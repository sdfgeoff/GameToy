pub use glow;
use glow::HasContext;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;

pub use tar;

pub mod config_file;
pub mod gamedata;
pub mod nodes;
mod quad;
pub mod shader;

mod gamestate;

pub use gamestate::GameState;

#[derive(Debug)]
pub enum GameToyError {
    DataLoadError(gamedata::GameDataError),
    QuadCreateError(quad::QuadError),
    NodeCreateError(String, nodes::NodeError),
    DuplicateNodeName(String),
    NoSuchNodeName(String),
    GetInputTextureFailed(String, nodes::NodeError),
    BindInputTextureFailed(String, nodes::NodeError),
    SelfReferenceSetupFailed(String, nodes::NodeError),

    /// Raised whenever the internal mapping of node->links does not have an entry for a specific node.
    InvalidLinkVec(),
}

type NodeRef = Rc<RefCell<Box<dyn nodes::Node>>>;

struct Link {
    start_node: NodeRef,
    start_output_slot: String,
    end_node: NodeRef,
    end_input_slot: String,
}

pub struct GameToy {
    game_state: GameState,

    // Everything is rendered on the same quad, so lets just chuck that here
    quad: quad::Quad,

    nodes: Vec<NodeRef>,

    links: HashMap<String, Vec<Link>>,

    pub output_node_maybe: Option<NodeRef>,
    enable_output: bool,

    resolution: [i32; 2],
    resolution_dirty: bool,
}

impl GameToy {
    pub fn new<R>(
        gl: &glow::Context,
        data: tar::Archive<R>,
        enable_output: bool,
    ) -> Result<Self, GameToyError>
    where
        R: Read,
    {
        let game_data = gamedata::GameData::from_tar(data).map_err(GameToyError::DataLoadError)?;

        let quad = quad::Quad::new(gl).map_err(GameToyError::QuadCreateError)?;

        let mut nodes: Vec<NodeRef> = vec![];
        let mut links = HashMap::new();

        let mut output_node_maybe = None;

        for node in game_data.config_file.graph.nodes.iter() {
            let new_node: NodeRef = match node {
                config_file::Node::RenderPass(pass_config) => {
                    let new_pass =
                        nodes::RenderPass::create_from_config(gl, &game_data, pass_config)
                            .map_err(|e| {
                                GameToyError::NodeCreateError(pass_config.name.clone(), e)
                            })?;
                    Rc::new(RefCell::new(Box::new(new_pass)))
                }
                config_file::Node::Output(output_config) => {
                    let output = nodes::Output::create_from_config(gl, output_config);
                    let output_node: NodeRef = Rc::new(RefCell::new(Box::new(output)));
                    output_node_maybe = Some(output_node.clone());
                    output_node
                }
                config_file::Node::Keyboard(key_config) => {
                    let keys = nodes::Keyboard::create_from_config(gl, key_config)
                        .map_err(|e| GameToyError::NodeCreateError(key_config.name.clone(), e))?;
                    Rc::new(RefCell::new(Box::new(keys)))
                }
                config_file::Node::Image(image_config) => {
                    let image = nodes::Image::create_from_config(gl, &game_data, image_config)
                        .map_err(|e| GameToyError::NodeCreateError(image_config.name.clone(), e))?;
                    Rc::new(RefCell::new(Box::new(image)))
                }
            };
            if links
                .insert(new_node.borrow().get_name().clone(), vec![])
                .is_some()
            {
                return Err(GameToyError::DuplicateNodeName(
                    new_node.borrow().get_name().clone(),
                ));
            }
            nodes.push(new_node);
        }

        for link in game_data.config_file.graph.links.iter() {
            let start_node = nodes
                .iter()
                .find(|x| x.borrow().get_name() == &link.start_node)
                .ok_or(GameToyError::NoSuchNodeName(link.start_node.clone()))?;
            let end_node = nodes
                .iter()
                .find(|x| x.borrow().get_name() == &link.end_node)
                .ok_or(GameToyError::NoSuchNodeName(link.end_node.clone()))?;

            let linkvec = links
                .get_mut(end_node.borrow().get_name())
                .ok_or(GameToyError::InvalidLinkVec())?;

            // Detects if a node self-references. The behaviour is up to the node to decide.
            if Rc::ptr_eq(start_node, end_node) {
                start_node
                    .borrow_mut()
                    .set_up_self_reference(&gl, &link.start_output_slot, &link.end_input_slot)
                    .map_err(|e| {
                        GameToyError::SelfReferenceSetupFailed(link.start_node.clone(), e)
                    })?;
            }

            linkvec.push(Link {
                start_node: start_node.clone(),
                start_output_slot: link.start_output_slot.clone(),
                end_node: end_node.clone(),
                end_input_slot: link.end_input_slot.clone(),
            })
        }

        if enable_output {
            unsafe {
                gl.clear_color(0.0, 1.0, 1.0, 1.0);
            }
        }

        Ok(Self {
            game_state: GameState::new(),
            quad,
            nodes,
            links,
            enable_output,
            output_node_maybe,
            resolution: [1920, 1080],
            resolution_dirty: false,
        })
    }

    // Perform a complete render
    // Requires the time as seconds past the unix epoch. Note that
    // if you pass this in as zero, the simulation will assume a frametime of
    // 60FPS.
    pub fn render(
        &mut self,
        gl: &glow::Context,
        time_since_unix_epoch: f64,
    ) -> Result<(), GameToyError> {
        self.game_state.update_times(time_since_unix_epoch);

        if self.resolution_dirty {
            for node in self.nodes.iter() {
                node.borrow_mut().update_resolution(gl, &self.resolution);
            }
            println!("Updating resolution {:?}", self.resolution);
            self.resolution_dirty = false;
        }

        unsafe {
            // Render all of the various passes
            for node in &self.nodes {
                if let Some(outnode) = &self.output_node_maybe {
                    if !self.enable_output && Rc::ptr_eq(node, outnode) {
                        continue;
                    }
                }
                let mut node_mut = node.borrow_mut();

                for link in self
                    .links
                    .get(node_mut.get_name())
                    .ok_or(GameToyError::InvalidLinkVec())?
                    .iter()
                {
                    assert!(Rc::ptr_eq(node, &link.end_node));

                    let tex = {
                        if Rc::ptr_eq(node, &link.start_node) {
                            // If we are having a node read from a previous version of itself
                            // we can't borrow it twice.
                            node_mut
                                .get_output_texture(&link.start_output_slot)
                                .map_err(|e| {
                                    GameToyError::GetInputTextureFailed(
                                        node_mut.get_name().clone(),
                                        e,
                                    )
                                })?
                        } else {
                            link.start_node
                                .borrow()
                                .get_output_texture(&link.start_output_slot)
                                .map_err(|e| {
                                    GameToyError::GetInputTextureFailed(
                                        node_mut.get_name().clone(),
                                        e,
                                    )
                                })?
                        }
                    };

                    node_mut
                        .set_input_texture(&link.end_input_slot, tex)
                        .map_err(|e| {
                            GameToyError::BindInputTextureFailed(node_mut.get_name().clone(), e)
                        })?;
                }
                node_mut.bind(gl, &self.quad, &self.game_state);
                gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
                node_mut.post_draw(gl, &self.game_state);
            }
        }

        self.game_state.clear_keys_dirty();
        self.game_state.update_key_tick();

        Ok(())
    }

    // Sets the size to render at
    pub fn resize(&mut self, x_pixels: u32, y_pixels: u32) {
        self.resolution = [x_pixels as i32, y_pixels as i32];
        self.resolution_dirty = true;
    }

    /// Used for keyboard input into GameToy.
    /// Note that the keycode should be equivalent to the Javascript one for
    /// compatibility
    pub fn set_key_state(&mut self, key_code: u32, key_down: bool) {
        self.game_state.set_key_state(key_code as usize, key_down);
    }

    /*
    fn destroy() {

    }

    fn set_key_state(keystate) {

    }

    fn set_mouse_state(mousestate) {

    }
    */
}

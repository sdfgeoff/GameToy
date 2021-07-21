pub use glow;
use glow::HasContext;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;

pub use tar;

mod config_file;
mod gamedata;
mod node;
mod output_node;
mod quad;
mod renderpass;
mod shader;

#[derive(Debug)]
pub enum GameToyError {
    DataLoadError(gamedata::GameDataError),
    QuadCreateError(quad::QuadError),
    RenderPassCreateError(String, renderpass::RenderPassError),
    DuplicateNodeName(String),
    NoSuchNodeName(String),
    GetInputTextureFailed(String, node::NodeError),
    BindInputTextureFailed(String, node::NodeError),
    SelfReferenceSetupFailed(String, node::NodeError),

    /// Raised whenever the internal mapping of node->links does not have an entry for a specific node.
    InvalidLinkVec(),
}

type NodeRef = Rc<RefCell<Box<dyn node::Node>>>;

struct Link {
    start_node: NodeRef,
    start_output_slot: String,
    end_node: NodeRef,
    end_input_slot: String,
}

pub struct GameToy {
    gl: glow::Context,

    // Time the last frame was rendered - used to calculate dt
    prev_render_time: f64,
    // Monotonic non-decreasing clock
    time_since_start: f64,

    // Everything is rendered on the same quad, so lets just chuck that here
    quad: quad::Quad,

    nodes: Vec<NodeRef>,

    links: HashMap<String, Vec<Link>>,

    resolution: [i32; 2],
}

impl GameToy {
    pub fn new<R>(gl: glow::Context, data: tar::Archive<R>) -> Result<Self, GameToyError>
    where
        R: Read,
    {
        let game_data = gamedata::GameData::from_tar(data).map_err(GameToyError::DataLoadError)?;

        let quad = quad::Quad::new(&gl).map_err(GameToyError::QuadCreateError)?;

        let mut nodes: Vec<NodeRef> = vec![];
        let mut links = HashMap::new();

        for node in game_data.config_file.graph.nodes.iter() {
            let new_node: NodeRef = match node {
                config_file::Node::RenderPass(pass_config) => {
                    let new_pass =
                        renderpass::RenderPass::create_from_config(&gl, &game_data, pass_config)
                            .map_err(|e| {
                                GameToyError::RenderPassCreateError(pass_config.name.clone(), e)
                            })?;
                    Rc::new(RefCell::new(Box::new(new_pass)))
                }
                config_file::Node::Texture(_texture_config) => {
                    unimplemented!()
                }
                config_file::Node::Output(output_config) => {
                    let output = output_node::OutputNode::create_from_config(&gl, output_config);
                    Rc::new(RefCell::new(Box::new(output)))
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

        unsafe {
            gl.clear_color(0.0, 1.0, 1.0, 1.0);
        }

        Ok(Self {
            gl,
            prev_render_time: 0.0,
            time_since_start: 0.0,
            quad,
            nodes,
            links,
            resolution: [1920, 1080],
        })
    }

    // Perform a complete render
    // Requires the time as seconds past the unix epoch. Note that
    // if you pass this in as zero, the simulation will assume a frametime of
    // 60FPS.
    pub fn render(&mut self, time_since_unix_epoch: f64) -> Result<(), GameToyError> {
        let dt = if self.prev_render_time == 0.0 {
            0.016
        } else {
            // Cap the dt at 0.0 - time should not move backwards
            f64::max(time_since_unix_epoch - self.prev_render_time, 0.0)
        };

        self.prev_render_time = time_since_unix_epoch;
        self.time_since_start += dt;

        unsafe {
            // Render all of the various passes
            for node in &self.nodes {
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
                node_mut.bind(&self.gl, &self.quad);

                self.gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            }
        }
        Ok(())
    }

    // Sets the size to render at
    pub fn resize(&mut self, x_pixels: u32, y_pixels: u32) {
        self.resolution = [x_pixels as i32, y_pixels as i32];

        for node in self.nodes.iter() {
            node.borrow_mut()
                .update_resolution(&self.gl, &self.resolution);
        }
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

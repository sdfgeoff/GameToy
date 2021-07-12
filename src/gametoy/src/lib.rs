pub use glow;
use glow::HasContext;
use std::io::Read;
pub use tar;

mod config_file;
mod gamedata;
mod quad;
mod shader;
mod node;
mod renderpass;
mod output_node;


pub struct GameToy {
    gl: glow::Context,

    // Time the last frame was rendered - used to calculate dt
    prev_render_time: f64,
    // Monotonic non-decreasing clock
    time_since_start: f64,

    // Everything is rendered on the same quad, so lets just chuck that here
    quad: quad::Quad,

    

    nodes: Vec<Box<dyn node::Node>>,

    resolution: [i32; 2],
}

impl GameToy {
    pub fn new<R>(gl: glow::Context, data: tar::Archive<R>) -> Self
    where
        R: Read,
    {
        let game_data = gamedata::GameData::from_tar(data).expect("Game Data Error");

        let quad = quad::Quad::new(&gl).expect("Failed to create quad");

        let mut nodes = vec![];
        for node in game_data.config_file.graph.nodes.iter() {
            match node {
                config_file::Node::RenderPass(pass_config) => {
                    let new_pass = renderpass::RenderPass::create_from_config(
                        &gl, 
                        &game_data,
                        pass_config
                    ).expect(&format!("Failed to create pass \"{}\" with error", pass_config.name));
                    let pass_trait: Box<dyn node::Node> = Box::new(new_pass);
                    nodes.push(pass_trait);
                }
                config_file::Node::Texture(_texture_config) => {
                    unimplemented!()
                }
                config_file::Node::Output(output_config) => {
                    let output = output_node::OutputNode::create_from_config(
                        &gl, 
                        output_config
                    );
                    nodes.push(Box::new(output));
                }
            }
        }

        unsafe {
            gl.clear_color(0.0, 1.0, 1.0, 1.0);
        }

        Self {
            gl,
            prev_render_time: 0.0,
            time_since_start: 0.0,
            quad,
            nodes,
            resolution: [1920, 1080],
        }
    }

    // Perform a complete render
    // Requires the time as seconds past the unix epoch. Note that
    // if you pass this in as zero, the simulation will assume a frametime of
    // 60FPS.
    pub fn render(&mut self, time_since_unix_epoch: f64) {
        let dt = if self.prev_render_time == 0.0 {
            0.016
        } else {
            // Cap the dt at 0.0 - time should not move backwards
            f64::max(time_since_unix_epoch - self.prev_render_time, 0.0)
        };

        self.prev_render_time = time_since_unix_epoch;
        self.time_since_start += dt;

        unsafe {
            self.quad.bind(&self.gl);

            // Render all of the various passes
            for node in self.nodes.iter_mut() {
                node.bind(&self.gl);
                self.gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            }
        }
        
    }

    // Sets the size to render at
    pub fn resize(&mut self, x_pixels: u32, y_pixels: u32) {
        self.resolution = [x_pixels as i32, y_pixels as i32];

        for node in self.nodes.iter_mut() {
            node.update_resolution(&self.gl, &self.resolution);
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


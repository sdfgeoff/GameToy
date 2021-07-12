use super::node;
use super::shader;
use super::config_file;

use glow::HasContext;

/// The output node writes to the screen!
pub struct OutputNode {
    pub name: String,
    pub resolution: [i32; 2],

    pub shader_program: shader::SimpleShader,
}

impl OutputNode {
    pub fn create_from_config(gl: &glow::Context, config: &config_file::OutputConfig) -> Self {


        let shader_program = shader::SimpleShader::new(
            &gl,
            include_str!("resources/shader.vert"),
            include_str!("resources/output_node.frag"),
        )
        .expect("Failed to create output shader");

        Self {
            name: config.name.clone(),
            resolution: [1920, 1080],
            shader_program,
        }
    }
}

impl node::Node for OutputNode {

    fn get_name(&self) -> &String {
        return &self.name
    }

    fn update_resolution(&mut self, _gl: &glow::Context, screen_resolution: &[i32; 2]) {
        self.resolution = screen_resolution.clone();
    }

    fn bind(&mut self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None); // Bind to the viewport - a framebuffer of None
            gl.viewport(0, 0, self.resolution[0], self.resolution[1]);
            self.shader_program.bind(gl);
        }
    }

}
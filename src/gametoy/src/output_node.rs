/*!
The Output Node
---------------

This node has a single input texture slot named "col". Whatever
texture is connected to this is draw directly onto the screen.

This buffer is always at the resolution of the users display.

!*/

use super::config_file;
use super::node;
use super::shader;

use glow::HasContext;

/// The output node writes to the screen!
pub struct OutputNode {
    pub name: String,
    pub resolution: [i32; 2],

    pub shader_program: shader::SimpleShader,

    pub output_tex_uniform: glow::UniformLocation,

    pub output_texture: Option<glow::Texture>,
}

impl OutputNode {
    pub fn create_from_config(gl: &glow::Context, config: &config_file::OutputConfig) -> Self {
        let shader_program = shader::SimpleShader::new(
            &gl,
            include_str!("resources/shader.vert"),
            include_str!("resources/output_node.frag"),
        )
        .expect("Failed to create output shader");

        let output_tex_uniform = unsafe { gl.get_uniform_location(shader_program.program, "col") }
            .expect("Output shader has no 'col' uniform");

        Self {
            name: config.name.clone(),
            resolution: [1920, 1080],
            output_tex_uniform,
            shader_program,
            output_texture: None,
        }
    }
}

impl node::Node for OutputNode {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn update_resolution(&mut self, _gl: &glow::Context, screen_resolution: &[i32; 2]) {
        self.resolution = screen_resolution.clone();
    }

    fn bind(&mut self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None); // Bind to the viewport - a framebuffer of None
            gl.viewport(0, 0, self.resolution[0], self.resolution[1]);
            self.shader_program.bind(gl);

            // Tell WebGL which texture unit we are configuring
            let texture_unit_id = 0;
            gl.active_texture(glow::TEXTURE0 + texture_unit_id);
            // Tell WebGL what texture to load into the texture unit
            gl.bind_texture(glow::TEXTURE_2D, self.output_texture);
            // Tell WebGL which uniform refers to this texture unit
            gl.uniform_1_i32(Some(&self.output_tex_uniform), texture_unit_id as i32);
        }
    }

    fn get_output_texture(&self, name: &String) -> Result<glow::Texture, node::NodeError> {
        Err(node::NodeError::NoSuchOutputTexture(name.clone()))
    }

    fn set_input_texture(
        &mut self,
        name: &String,
        texture: glow::Texture,
    ) -> Result<(), node::NodeError> {
        if name == "col" {
            self.output_texture = Some(texture);
            Ok(())
        } else {
            Err(node::NodeError::NoSuchInputTexture(name.clone()))
        }
    }
}

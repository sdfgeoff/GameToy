/*!
The Renderpass Node
-------------------

The renderpass node is the heart of gametoy. It is responsible for
taking a bunch of input textures and turning them into a bunch of
output textures. It does this by running a GLSL shader.

The configuration for this node allows controlling the names of the
input and output slots as well as their pixel formats, and the resolution.


TODOs: 
 - Double buffering so that a node can read from itself

!*/

use super::config_file;
use super::gamedata::GameData;
use super::node;
use super::shader::{ShaderError, SimpleShader};
use glow::HasContext;

use std::collections::HashMap;

/// Contains data necessary for rendering this renderpass in isolation
/// It does not contain information about where input textures come from.
pub struct RenderPass {
    /// What this renderpass is called. This is a human-readable name that must be
    /// unique in the program
    name: String,

    /// The shader program that is used to render this renderpass
    shader_program: SimpleShader,

    /// The framebuffer that the shader renders into
    framebuffer: glow::Framebuffer,

    /// Stores the current resolution of this renderpass. This is used
    resolution: [i32; 2],
    
    /// Controls how the RenderPass behaves when calling `update_resolution`
    resolution_scaling_mode: config_file::ResolutionScalingMode,

    /// The textures that this renderpass outputs. Other nodes can use these as inputs.
    output_textures: HashMap<String, OutputTexture>,

    /// The textures that this renderpass reads from when rendering.
    input_textures: HashMap<String, Option<glow::Texture>>,
}

/// Container for a texture and it's configuration.
struct OutputTexture {
    tex: glow::Texture,
    config: config_file::OutputBufferConfig,
}

#[derive(Debug)]
pub enum RenderPassError {
    /// The GPU failed to allocate the framebuffer
    CreateFramebufferFailed(String),

    /// The GPU failed to allocate a texture
    CreateTextureFailed(String),

    /// The text files that should contain the shader source code
    /// do not exist in the supplied GameData
    MissingShaderSource(String),

    /// This renderpass has two input slots with the same name
    DuplicateInputSlotName(String),
    
    /// This renderpass has two output slots with the same name
    DuplicateOutputSlotName(String),

    /// There is no shader defined for this renderpass!
    NoShader,

    /// Shader failed to compile/link etc.
    ShaderError(ShaderError),
}

impl RenderPass {
    pub fn create_from_config(
        gl: &glow::Context,
        gamedata: &GameData,
        config: &config_file::RenderPassConfig,
    ) -> Result<Self, RenderPassError> {
        // Load and compile the shader
        let mut fragment_source = String::new();
        for shader_path in config.fragment_shader_paths.iter() {
            let source = gamedata.shader_sources.get(shader_path).ok_or(
                RenderPassError::MissingShaderSource(shader_path.to_string()),
            )?;
            fragment_source += source;
        }
        if fragment_source.len() == 0 {
            Err(RenderPassError::NoShader)?;
        }
        let shader_program = SimpleShader::new(
            gl,
            include_str!("./resources/shader.vert"),
            &fragment_source,
        )
        .map_err(RenderPassError::ShaderError)?;

        // Create the framebuffer
        let framebuffer = unsafe {
            gl.create_framebuffer()
                .map_err(RenderPassError::CreateFramebufferFailed)?
        };
        // Set it up so we are operating on our framebuffer and have a texture unit to play with
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
            gl.active_texture(glow::TEXTURE0);
        }

        let mut attachment_id = 0;
        let mut buffers = vec![];

        let resolution = match config.resolution_scaling_mode {
            config_file::ResolutionScalingMode::Fixed(width, height) => [width, height],
            config_file::ResolutionScalingMode::ViewportScale(x_percent, y_percent) => {
                [(1920.0 * x_percent) as i32, (1080.0 * y_percent) as i32]
            }
        };

        let mut output_textures = HashMap::new();

        for output_texture_slot in config.output_texture_slots.iter() {
            unsafe {
                let new_tex = gl
                    .create_texture()
                    .map_err(RenderPassError::CreateTextureFailed)?;
                if output_textures
                    .insert(
                        output_texture_slot.name.clone(),
                        OutputTexture {
                            tex: new_tex,
                            config: output_texture_slot.clone(),
                        },
                    )
                    .is_some()
                {
                    return Err(RenderPassError::DuplicateOutputSlotName(
                        output_texture_slot.name.clone(),
                    ));
                }

                gl.bind_texture(glow::TEXTURE_2D, Some(new_tex));
                //gl.pixel_storei(flow::UNPACK_FLIP_Y_WEBGL, 0);

                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MAG_FILTER,
                    glow::NEAREST as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MIN_FILTER,
                    glow::NEAREST as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_WRAP_S,
                    glow::CLAMP_TO_EDGE as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_WRAP_T,
                    glow::CLAMP_TO_EDGE as i32,
                );

                match config.resolution_scaling_mode {
                    config_file::ResolutionScalingMode::Fixed(_, _) => {
                        // We know this isn't going to change, so we can use tex_storage_2d
                        gl.tex_storage_2d(
                            glow::TEXTURE_2D,
                            1,
                            output_texture_slot.format.to_sized_internal_format(),
                            resolution[0],
                            resolution[1],
                        );
                    }
                    config_file::ResolutionScalingMode::ViewportScale(_, _) => {
                        // For textures that can change size we use TexImage2d
                        gl.tex_image_2d(
                            glow::TEXTURE_2D,
                            0,
                            output_texture_slot.format.to_sized_internal_format() as i32,
                            resolution[0],
                            resolution[1],
                            0,
                            output_texture_slot.format.to_format(), // If we were passing in an existing image into data, this would be meaningful
                            output_texture_slot.format.to_type(), // If we were passing in an existing image into data, this would be meaningful
                            None, // but we are passing in None here, so the above two values are ignored.
                        );
                    }
                }

                let attachment = color_attachment_int_to_gl(attachment_id);
                buffers.push(attachment);

                gl.framebuffer_texture_2d(
                    glow::FRAMEBUFFER,
                    attachment,
                    glow::TEXTURE_2D,
                    Some(new_tex),
                    0,
                );
                attachment_id += 1;
            }
        }

        let mut input_textures = HashMap::new();
        for input_texture_slot in config.output_texture_slots.iter() {
            if input_textures
                .insert(input_texture_slot.name.clone(), None)
                .is_some()
            {
                return Err(RenderPassError::DuplicateInputSlotName(
                    input_texture_slot.name.clone(),
                ));
            }
        }

        unsafe {
            gl.draw_buffers(&buffers);
        }

        Ok(Self {
            name: config.name.clone(),
            shader_program,
            framebuffer,
            resolution,
            input_textures,
            resolution_scaling_mode: config.resolution_scaling_mode.clone(),
            output_textures,
        })
    }
}

impl node::Node for RenderPass {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn bind(&mut self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
            gl.viewport(0, 0, self.resolution[0], self.resolution[1]);
            self.shader_program.bind(gl);
        }
    }

    fn update_resolution(&mut self, gl: &glow::Context, screen_resolution: &[i32; 2]) {
        match self.resolution_scaling_mode {
            config_file::ResolutionScalingMode::Fixed(_, _) => {}
            config_file::ResolutionScalingMode::ViewportScale(x_percent, y_percent) => {
                self.resolution = [
                    ((screen_resolution[0] as f32) * x_percent) as i32,
                    ((screen_resolution[1] as f32) * y_percent) as i32,
                ];

                // resize textures inside the buffer
                unsafe {
                    for (_texname, tex) in self.output_textures.iter() {
                        gl.bind_texture(glow::TEXTURE_2D, Some(tex.tex));

                        gl.tex_image_2d(
                            glow::TEXTURE_2D,
                            0,
                            tex.config.format.to_sized_internal_format() as i32,
                            self.resolution[0],
                            self.resolution[1],
                            0,
                            tex.config.format.to_format(), // If we were passing in an existing image into data, this would be meaningful
                            tex.config.format.to_type(), // If we were passing in an existing image into data, this would be meaningful
                            None, // but we are passing in None here, so the above two values are ignored.
                        );
                    }
                }
            }
        }
    }

    fn get_output_texture(&self, name: &String) -> Result<glow::Texture, node::NodeError> {
        self.output_textures
            .get(name)
            .map(|x| x.tex)
            .ok_or(node::NodeError::NoSuchOutputTexture(name.clone()))
        // TODO: double buffering will be more involved here
    }

    fn set_input_texture(
        &mut self,
        name: &String,
        texture: glow::Texture,
    ) -> Result<(), node::NodeError> {
        match self.input_textures.get_mut(name) {
            Some(slot) => {
                slot.replace(texture);
                Ok(())
            }
            None => Err(node::NodeError::NoSuchInputTexture(name.clone())),
        }
    }
}

fn color_attachment_int_to_gl(int: u8) -> u32 {
    match int {
        0 => glow::COLOR_ATTACHMENT0 as u32,
        1 => glow::COLOR_ATTACHMENT1 as u32,
        2 => glow::COLOR_ATTACHMENT2 as u32,
        3 => glow::COLOR_ATTACHMENT3 as u32,
        4 => glow::COLOR_ATTACHMENT4 as u32,
        5 => glow::COLOR_ATTACHMENT5 as u32,
        6 => glow::COLOR_ATTACHMENT6 as u32,
        7 => glow::COLOR_ATTACHMENT7 as u32,
        8 => glow::COLOR_ATTACHMENT8 as u32,
        9 => glow::COLOR_ATTACHMENT9 as u32,
        10 => glow::COLOR_ATTACHMENT10 as u32,
        _ => panic!("Too many color attachments!"),
    }
}

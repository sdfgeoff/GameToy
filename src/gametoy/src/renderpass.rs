use super::config_file;
use super::node::Node;
use glow::{HasContext};
use super::gamedata::GameData;
use super::shader::{SimpleShader, ShaderError};

use std::collections::HashMap;

pub struct RenderPass {
    name: String,
    shader_program: SimpleShader,
    framebuffer: glow::Framebuffer,
    resolution: [i32; 2],

    resolution_scaling_mode: config_file::ResolutionScalingMode,

    output_textures: HashMap<String, OutputTexture>
}

struct OutputTexture {
    tex: glow::Texture,
    config: config_file::OutputBufferConfig,
}

#[derive(Debug)]
pub enum RenderPassError {
    CreateFramebufferFailed(String),
    CreateTextureFailed(String),
    MissingShaderSource(String),
    NoShader,
    ShaderError(ShaderError)
}

impl RenderPass {
    pub fn create_from_config(gl: &glow::Context, gamedata: &GameData, config: &config_file::RenderPassConfig) -> Result<Self, RenderPassError> {

        // Load and compile the shader
        let mut fragment_source = String::new();
        for shader_path in config.fragment_shader_paths.iter() {
            let source = gamedata.shader_sources.get(shader_path).ok_or(RenderPassError::MissingShaderSource(shader_path.to_string()))?;
            fragment_source += source;
        }
        if fragment_source.len() == 0 {
            Err(RenderPassError::NoShader)?;
        }
        let shader_program = SimpleShader::new(gl, include_str!("./resources/shader.vert"), &fragment_source).map_err(RenderPassError::ShaderError)?;


        // Create the framebuffer
        let framebuffer = unsafe {
            gl.create_framebuffer().map_err(RenderPassError::CreateFramebufferFailed)?
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
            config_file::ResolutionScalingMode::ViewportScale(x_percent, y_percent) => [
                (1920.0 * x_percent) as i32, 
                (1080.0 * y_percent) as i32,
            ],
        };

        let mut output_textures = HashMap::new();

        for output_texture_slot in config.output_texture_slots.iter() {
            unsafe {
                let new_tex = gl.create_texture().map_err(RenderPassError::CreateTextureFailed)?;
                output_textures.insert(
                    output_texture_slot.name.clone(), 
                    OutputTexture {
                        tex: new_tex,
                        config: output_texture_slot.clone()
                    }
                );

                gl.bind_texture(glow::TEXTURE_2D, Some(new_tex));
                //gl.pixel_storei(flow::UNPACK_FLIP_Y_WEBGL, 0);

                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

                match config.resolution_scaling_mode {
                    config_file::ResolutionScalingMode::Fixed(_, _) => {
                        // We know this isn't going to change, so we can use tex_storage_2d
                        gl.tex_storage_2d(
                            glow::TEXTURE_2D, 
                            1, 
                            output_texture_slot.format.to_gl_const(), 
                            resolution[0], 
                            resolution[1]
                        );
                        
                    }
                    config_file::ResolutionScalingMode::ViewportScale(_, _) => {
                        // For textures that can change size we use TexImage2d
                        gl.tex_image_2d(
                            glow::TEXTURE_2D, 
                            0, 
                            output_texture_slot.format.to_gl_const() as i32, 
                            resolution[0], 
                            resolution[1],
                            0,
                            glow::RGBA_INTEGER, // If we were passing in an existing image into data, this would be meaningful
                            glow::UNSIGNED_BYTE, // If we were passing in an existing image into data, this would be meaningful
                            None // but we are passing in None here, so the above two values are ignored.
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

        unsafe {
            gl.draw_buffers(&buffers);
        }




        Ok(Self {
            name: config.name.clone(),
            shader_program,
            framebuffer,
            resolution,
            resolution_scaling_mode: config.resolution_scaling_mode.clone(),
            output_textures,
        })
    }


}


impl Node for RenderPass {
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
            config_file::ResolutionScalingMode::Fixed(_, _) => {},
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
                            tex.config.format.to_gl_const() as i32,
                            self.resolution[0], 
                            self.resolution[1],
                            0,
                            glow::RGBA_INTEGER, // If we were passing in an existing image into data, this would be meaningful
                            glow::UNSIGNED_BYTE, // If we were passing in an existing image into data, this would be meaningful
                            None // but we are passing in None here, so the above two values are ignored.
                        );

                    }
                }
            }
        }

    }
}



fn color_attachment_int_to_gl(int: u8) -> u32{
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
        _ => panic!("Too many color attachments!")
    }
}

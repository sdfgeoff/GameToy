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

    /// Stores the current resolution of this renderpass. This is used both by the shader
    /// via the iResolution variable and to set the glViewport
    resolution: [i32; 2],

    frame: u32,

    /// Configuration of this renderpass. This is needed as some information (eg scaling mode) is needed
    /// at runtime.
    config: config_file::RenderPassConfig,

    /// The textures that this renderpass reads from when rendering.
    input_textures: HashMap<String, Option<glow::Texture>>,

    /// The framebuffer that the shader renders into
    framebuffer: glow::Framebuffer,

    /// The textures that this renderpass outputs. Other nodes can use these as inputs.
    output_textures: HashMap<String, OutputTexture>,

    /// If this render-pass is self-referential, then we need another framebuffer to render into
    back_framebuffer: Option<glow::Framebuffer>,

    /// If this render-pass is self-referential, then we need another lot of textures to render into.
    back_output_textures: Option<HashMap<String, OutputTexture>>,

    /// Indicates if the textures to use are the front or back set. Whenever there is a
    /// `back_frambuffer`, this value will be toggled with each call to bind()
    use_back_buffers: bool,

    uniform_map: HashMap<String, glow::UniformLocation>,
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

        let shader_preamble = include_str!("./resources/renderpass_static.frag");

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

        let full_shader_code = String::new() + shader_preamble + &fragment_source;

        let mut shader_program = SimpleShader::new(
            gl,
            include_str!("./resources/shader.vert"),
            &full_shader_code,
        )
        .map_err(RenderPassError::ShaderError)?;
        shader_program.bind(gl);

        let resolution = match config.resolution_scaling_mode {
            config_file::ResolutionScalingMode::Fixed(width, height) => [width, height],
            config_file::ResolutionScalingMode::ViewportScale(x_percent, y_percent) => {
                [(1920.0 * x_percent) as i32, (1080.0 * y_percent) as i32]
            }
        };

        let (framebuffer, output_textures) =
            create_framebuffer_and_textures(gl, config, resolution)?;

        let mut input_textures = HashMap::new();
        for input_texture_slot in config.input_texture_slots.iter() {
            if input_textures
                .insert(input_texture_slot.name.clone(), None)
                .is_some()
            {
                return Err(RenderPassError::DuplicateInputSlotName(
                    input_texture_slot.name.clone(),
                ));
            }
        }

        let mut uniform_map = HashMap::new();

        insert_uniform_if_exists(
            gl,
            &mut uniform_map,
            &shader_program.program,
            "iResolution".to_string(),
        );
        insert_uniform_if_exists(
            gl,
            &mut uniform_map,
            &shader_program.program,
            "iTime".to_string(),
        );
        insert_uniform_if_exists(
            gl,
            &mut uniform_map,
            &shader_program.program,
            "iTimeDelta".to_string(),
        );
        insert_uniform_if_exists(
            gl,
            &mut uniform_map,
            &shader_program.program,
            "iFrame".to_string(),
        );
        insert_uniform_if_exists(
            gl,
            &mut uniform_map,
            &shader_program.program,
            "iMouse".to_string(),
        );
        insert_uniform_if_exists(
            gl,
            &mut uniform_map,
            &shader_program.program,
            "iDate".to_string(),
        );

        Ok(Self {
            name: config.name.clone(),
            shader_program,
            framebuffer,
            resolution,
            input_textures,
            config: config.clone(),
            output_textures,
            back_framebuffer: None,
            back_output_textures: None,
            use_back_buffers: false,
            frame: 0,
            uniform_map,
        })
    }
}

fn insert_uniform_if_exists(
    gl: &glow::Context,
    uniform_map: &mut HashMap<String, glow::UniformLocation>,
    program: &glow::Program,
    uniform_name: String,
) {
    let uniform_location = unsafe { gl.get_uniform_location(*program, &uniform_name) };
    if let Some(loc) = uniform_location {
        uniform_map.insert(uniform_name, loc);
    }
}

fn create_framebuffer_and_textures(
    gl: &glow::Context,
    config: &config_file::RenderPassConfig,
    resolution: [i32; 2],
) -> Result<(glow::Framebuffer, HashMap<String, OutputTexture>), RenderPassError> {
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

    unsafe {
        gl.draw_buffers(&buffers);
    }

    Ok((framebuffer, output_textures))
}

impl node::Node for RenderPass {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn bind(&mut self, gl: &glow::Context, quad: &super::quad::Quad) {
        unsafe {
            if self.back_framebuffer.is_some() {
                // If we have a back framebuffer, switch it for next time.
                self.use_back_buffers = !self.use_back_buffers;
            }

            if self.use_back_buffers {
                assert!(self.back_framebuffer.is_some());
                gl.bind_framebuffer(glow::FRAMEBUFFER, self.back_framebuffer);
            } else {
                gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
            }

            gl.viewport(0, 0, self.resolution[0], self.resolution[1]);
            self.shader_program.bind(gl);

            // Builtin Uniforms
            if let Some(loc) = self.uniform_map.get("iResolution") {
                gl.uniform_3_f32(
                    Some(&loc),
                    self.resolution[0] as f32,
                    self.resolution[1] as f32,
                    1.0,
                );
            }
            if let Some(loc) = self.uniform_map.get("iFrame") {
                gl.uniform_1_u32(Some(&loc), self.frame);
            }

            quad.bind(gl, self.shader_program.attrib_vertex_positions);
        }
        self.frame = self.frame.overflowing_add(1).0;
    }

    fn update_resolution(&mut self, gl: &glow::Context, screen_resolution: &[i32; 2]) {
        match self.config.resolution_scaling_mode {
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

                    // if we are double-buffering, also resize our other-output-textures
                    if let Some(output_textures) = &self.back_output_textures {
                        for (_texname, tex) in output_textures.iter() {
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
    }

    fn get_output_texture(&self, name: &String) -> Result<glow::Texture, node::NodeError> {
        if self.use_back_buffers {
            // If we are rendering to the front textures this frame, return the back textures
            self.back_output_textures
                .as_ref()
                .expect("Trying to use backbuffers when they don't exist. Invalid state")
                .get(name)
                .map(|x| x.tex)
                .ok_or(node::NodeError::NoSuchOutputTexture(name.clone()))
        } else {
            self.output_textures
                .get(name)
                .map(|x| x.tex)
                .ok_or(node::NodeError::NoSuchOutputTexture(name.clone()))
        }
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

    fn set_up_self_reference(
        &mut self,
        gl: &glow::Context,
        _: &String,
        _: &String,
    ) -> Result<(), node::NodeError> {
        let (framebuffer, output_textures) =
            create_framebuffer_and_textures(gl, &self.config, self.resolution)
                .expect("Failed to create backbuffer");

        self.back_framebuffer = Some(framebuffer);
        self.back_output_textures = Some(output_textures);
        Ok(())
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

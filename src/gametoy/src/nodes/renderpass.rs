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

use super::node;
use super::NodeError;
use crate::config_file;
use crate::gamedata::GameData;
use crate::quad::Quad;
use crate::shader::SimpleShader;
use crate::GameState;
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

    uniform_map: HashMap<String, glow::UniformLocation>,
}

/// Container for a texture and it's configuration.
struct OutputTexture {
    tex: glow::Texture,
    config: config_file::OutputBufferConfig,
}

impl OutputTexture {
    // Creates and sets up magnification filters for a texture.
    // IMPORTANT: does not set up storage for the texture. You will need
    // to re-bind the texture and do a call to `gl.tex_image_2d` or `gl.tex_storage_2d`
    fn new(
        gl: &glow::Context,
        config: &config_file::OutputBufferConfig,
    ) -> Result<Self, NodeError> {
        
        let new_tex = unsafe {
            gl.create_texture()
                .map_err(NodeError::CreateTextureFailed)?
        };

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(new_tex));

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            ); 
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::REPEAT as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::REPEAT as i32,
            );

            assert_eq!(gl.get_error(), glow::NO_ERROR);
        }

        Ok(Self {
            tex: new_tex,
            config: config.clone(),
        })
    }

    fn resize(&self, gl: &glow::Context, resolution: &[i32; 2]) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.tex));

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                self.config.format.to_sized_internal_format() as i32,
                resolution[0],
                resolution[1],
                0,
                self.config.format.to_format(), // If we were passing in an existing image into data, this would be meaningful
                self.config.format.to_type(), // If we were passing in an existing image into data, this would be meaningful
                None, // but we are passing in None here, so the above two values are ignored.
            );
            gl.generate_mipmap(glow::TEXTURE_2D);
        }
    }

    fn generate_mip(&self, gl: &glow::Context) {
        if self.config.generate_mipmap {
            unsafe {
                gl.active_texture(glow::TEXTURE0);
                gl.bind_texture(glow::TEXTURE_2D, Some(self.tex));
                gl.generate_mipmap(glow::TEXTURE_2D);
            }
        }
    }
}

impl RenderPass {
    pub fn create_from_config(
        gl: &glow::Context,
        gamedata: &GameData,
        config: &config_file::RenderPassConfig,
    ) -> Result<Self, NodeError> {
        // First we create the framebuffer and output textures that this shader
        // will render into.
        let resolution = match config.resolution_scaling_mode {
            config_file::ResolutionScalingMode::Fixed(width, height) => [width, height],
            config_file::ResolutionScalingMode::ViewportScale(x_percent, y_percent) => {
                [(1920.0 * x_percent) as i32, (1080.0 * y_percent) as i32]
            }
        };
        let (framebuffer, output_textures) =
            create_framebuffer_and_textures(gl, config, resolution)?;

        let mut shader_program = SimpleShader::new(
            gl,
            include_str!("../resources/shader.vert"),
            &generate_shader_text(config, gamedata)?,
        )
        .map_err(NodeError::ShaderError)?;
        shader_program.bind(gl);

        // If we know what uniforms exist in advance we can replace lots of GL calls with
        // a hashmap lookup.
        let mut uniform_map = HashMap::new();

        let prog = &shader_program.program;
        insert_uniform_if_exists(gl, &mut uniform_map, prog, "iResolution".to_string());
        insert_uniform_if_exists(gl, &mut uniform_map, prog, "iTime".to_string());
        insert_uniform_if_exists(gl, &mut uniform_map, prog, "iTimeDelta".to_string());
        insert_uniform_if_exists(gl, &mut uniform_map, prog, "iFrame".to_string());
        insert_uniform_if_exists(gl, &mut uniform_map, prog, "iMouse".to_string());
        insert_uniform_if_exists(gl, &mut uniform_map, prog, "iDate".to_string());

        // Make sure that our input textures are known
        let mut input_textures = HashMap::new();
        for input_texture_slot in config.input_texture_slots.iter() {
            insert_uniform_if_exists(gl, &mut uniform_map, prog, input_texture_slot.name.clone());

            if input_textures
                .insert(input_texture_slot.name.clone(), None)
                .is_some()
            {
                return Err(NodeError::DuplicateInputSlotName(
                    input_texture_slot.name.clone(),
                ));
            }
        }

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
            frame: 0,
            uniform_map,
        })
    }
}

impl node::Node for RenderPass {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn bind(&mut self, gl: &glow::Context, quad: &Quad, game_state: &GameState) {
        unsafe {
            if self.back_framebuffer.is_some() && self.frame % 2 == 0 {
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
            if let Some(loc) = self.uniform_map.get("iTimeDelta") {
                gl.uniform_1_f32(Some(&loc), game_state.time_delta as f32);
            }
            if let Some(loc) = self.uniform_map.get("iTime") {
                gl.uniform_1_f32(Some(&loc), game_state.time_since_start as f32);
            }
            if let Some(loc) = self.uniform_map.get("iDate") {
                gl.uniform_4_f32(
                    Some(&loc),
                    game_state.date[0] as f32,
                    game_state.date[1] as f32,
                    game_state.date[2] as f32,
                    game_state.date[3] as f32,
                );
            }

            // Textures
            for (texture_id, (texture_name, texture)) in self.input_textures.iter().enumerate() {
                gl.active_texture(texture_unit_id_to_gl(texture_id as u32));
                gl.bind_texture(glow::TEXTURE_2D, *texture);
                // Tell WebGL which uniform refers to this texture unit
                if let Some(loc) = self.uniform_map.get(texture_name) {
                    gl.uniform_1_i32(Some(loc), texture_id as i32);
                } else {
                    // TODO: Raise a warning somehow?
                    // panic!("No Uniform for input texture");
                }
            }

            quad.bind(gl, self.shader_program.attrib_vertex_positions);
        }
        self.frame = self.frame.overflowing_add(1).0;
    }

    fn post_draw(&mut self, gl: &glow::Context, game_state: &GameState) -> Result<(), NodeError> {
        if self.back_output_textures.is_some() && self.frame % 2 == 1  {
            for outtex in self.back_output_textures.as_ref().unwrap().values() {
                outtex.generate_mip(gl);
            }
        } else {
            for outtex in self.output_textures.values() {
                outtex.generate_mip(gl);
            }
        }
        Ok(())

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
                for tex in self.output_textures.values() {
                    tex.resize(gl, screen_resolution);
                }

                // if we are double-buffering, also resize our other-output-textures
                if let Some(output_textures) = &self.back_output_textures {
                    for tex in output_textures.values() {
                        tex.resize(gl, screen_resolution);
                    }
                }
            }
        }
    }

    fn get_output_texture(&self, name: &String) -> Result<glow::Texture, node::NodeError> {
        if self.back_output_textures.is_some() && self.frame % 2 == 1 {
            // If we are rendering to the front textures this frame, return the back textures
            self.back_output_textures
                .as_ref()
                .unwrap()
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
        // this function can be called multiple times if there are multiple self-references.
        // so we need to guard against creating lots of back-buffers
        if self.back_framebuffer.is_none() {
            let (framebuffer, output_textures) =
                create_framebuffer_and_textures(gl, &self.config, self.resolution)
                    .expect("Failed to create backbuffer");

            self.back_framebuffer = Some(framebuffer);
            self.back_output_textures = Some(output_textures);
        }
        Ok(())
    }
}

/// Attempts to fetch a uniform's location from a shader program and insert it into a hashmap
/// Does nothing if the uniform does not exist.
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
) -> Result<(glow::Framebuffer, HashMap<String, OutputTexture>), NodeError> {
    // Create the framebuffer
    let framebuffer = unsafe {
        gl.create_framebuffer()
            .map_err(NodeError::CreateFramebufferFailed)?
    };
    // Set it up so we are operating on our framebuffer and have a texture unit to play with
    unsafe {
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
        gl.active_texture(glow::TEXTURE0);
    }

    let mut buffers = vec![];
    let mut output_textures = HashMap::new();

    for (attachment_id, output_texture_slot) in config.output_texture_slots.iter().enumerate() {
        let output_tex = OutputTexture::new(gl, output_texture_slot)?;

        let attachment = color_attachment_int_to_gl(attachment_id as u32);
        buffers.push(attachment);

        let levels = {
            if output_texture_slot.generate_mipmap {
                (resolution[0] as f32).log2().ceil() as i32
            } else {
                1
            }
        };

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(output_tex.tex));
            match config.resolution_scaling_mode {
                config_file::ResolutionScalingMode::Fixed(_, _) => {
                    // We know this isn't going to change, so we can use tex_storage_2d
                    gl.tex_storage_2d(
                        glow::TEXTURE_2D,
                        levels,
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
                    gl.generate_mipmap(glow::TEXTURE_2D);
                }
            }

            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                attachment,
                glow::TEXTURE_2D,
                Some(output_tex.tex),
                0,
            );
        }

        if output_textures
            .insert(output_tex.config.name.clone(), output_tex)
            .is_some()
        {
            return Err(NodeError::DuplicateOutputSlotName(
                output_texture_slot.name.clone(),
            ));
        }
    }

    unsafe {
        gl.draw_buffers(&buffers);
    }

    Ok((framebuffer, output_textures))
}

/// Returns the code that is inserted before the users GLSL. This includes
/// texture uniforms, the shader version etc.
fn generate_shader_text(
    config: &config_file::RenderPassConfig,
    gamedata: &GameData,
) -> Result<String, NodeError> {
    let mut shader_text = String::new();

    // Static things such as the shader version and "global" uniforms
    shader_text += include_str!("../resources/renderpass_static.frag");

    // Generate some shader source to represent the output textures
    for (slot_id, output_texture_slot) in config.output_texture_slots.iter().enumerate() {
        shader_text += &format!(
            "layout(location={}) out vec{} {};\n",
            slot_id,
            output_texture_slot.format.to_channel_count(),
            output_texture_slot.name
        );
    }

    // Generate some shader source to represent the input textures
    for input_texture_slot in config.input_texture_slots.iter() {
        shader_text += &format!("uniform sampler2D {};\n", input_texture_slot.name);
    }

    let preamble_length = shader_text.len();
    // Now we can assemble all the shader source into a single file and compile it
    for shader_path in config.fragment_shader_paths.iter() {
        let source = gamedata
            .shader_sources
            .get(shader_path)
            .ok_or(NodeError::MissingResource(shader_path.to_string()))?;
        shader_text += source;
    }
    // Nothing was added to the shader when reading from disk, so there is no
    // actual rendering going to be performed here.
    if shader_text.len() == preamble_length {
        Err(NodeError::NoShader)?;
    }

    Ok(shader_text)
}

fn color_attachment_int_to_gl(int: u32) -> u32 {
    assert!(int <= 10);
    glow::COLOR_ATTACHMENT0 + int
}

fn texture_unit_id_to_gl(int: u32) -> u32 {
    assert!(int <= 32);
    glow::TEXTURE0 + int
}

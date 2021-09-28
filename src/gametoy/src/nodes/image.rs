use super::{Node, NodeError};
use crate::quad::Quad;
use crate::GameState;
use glow::HasContext;
use png::{BitDepth, ColorType};

use crate::config_file::OutputBufferFormat;

use png;

pub struct Image {
    pub name: String,
    pub texture: glow::Texture,
}

impl Image {
    pub const OUTPUT_BUFFER_NAME: &'static str = "tex";

    pub fn create_from_config(
        gl: &glow::Context,
        gamedata: &crate::gamedata::GameData,
        config: &crate::config_file::ImageConfig,
    ) -> Result<Self, NodeError> {
        let new_tex = unsafe {
            gl.create_texture()
                .map_err(NodeError::CreateTextureFailed)?
        };

        let data = gamedata
            .textures
            .get(&config.path)
            .ok_or(NodeError::MissingResource(config.path.to_string()))?;

        let decoder = png::Decoder::new(data.as_slice());
        let (info, mut reader) = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; info.buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        reader.next_frame(&mut buf).unwrap();

        let tex_format = match reader.output_color_type() {
            (ColorType::RGB, BitDepth::Eight) => OutputBufferFormat::RGB8,
            (ColorType::RGB, BitDepth::Sixteen) => OutputBufferFormat::RGBA16UI,
            (ColorType::RGBA, BitDepth::Eight) => OutputBufferFormat::RGBA8,
            (ColorType::RGBA, BitDepth::Sixteen) => OutputBufferFormat::RGBA16UI,
            (ColorType::Grayscale, BitDepth::Eight) => OutputBufferFormat::R8,
            (ColorType::Grayscale, BitDepth::Sixteen) => OutputBufferFormat::R16UI,
            (_, _) => unimplemented!("Unsupported PNG Pixel Type"),
        };

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(new_tex));

            gl.tex_storage_2d(
                glow::TEXTURE_2D,
                1,
                tex_format.to_sized_internal_format(),
                info.width as i32,
                info.height as i32,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
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

            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                info.width as i32,
                info.height as i32,
                tex_format.to_format(),
                tex_format.to_type(),
                glow::PixelUnpackData::Slice(&buf),
            );
        }

        Ok(Self {
            name: config.name.clone(),
            texture: new_tex,
        })
    }
}

impl Node for Image {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn update_resolution(&mut self, _gl: &glow::Context, _screen_resolution: &[i32; 2]) {}

    fn bind(&mut self, _gl: &glow::Context, _quad: &Quad, _game_state: &GameState) {}

    fn get_output_texture(&self, name: &String) -> Result<glow::Texture, NodeError> {
        if name == Self::OUTPUT_BUFFER_NAME {
            Ok(self.texture)
        } else {
            Err(NodeError::NoSuchOutputTexture(name.clone()))
        }
    }

    fn set_input_texture(
        &mut self,
        name: &String,
        _texture: glow::Texture,
    ) -> Result<(), NodeError> {
        Err(NodeError::NoSuchInputTexture(name.clone()))
    }
}

use super::{Node, NodeError};
use crate::quad::Quad;
use crate::GameState;
use glow::HasContext;

pub struct Keyboard {
    name: String,
    dirty: bool,
    texture: glow::Texture,
}

const TEX_FORMAT: crate::config_file::OutputBufferFormat =
    crate::config_file::OutputBufferFormat::R8_SNORM;

impl Keyboard {
    pub const OUTPUT_BUFFER_NAME: &'static str = "tex";

    pub fn create_from_config(
        gl: &glow::Context,
        config: &crate::config_file::KeyboardConfig,
    ) -> Result<Self, NodeError> {
        let new_tex = unsafe {
            gl.create_texture()
                .map_err(NodeError::CreateTextureFailed)?
        };

        unsafe {
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

            gl.tex_storage_2d(
                glow::TEXTURE_2D,
                1, // Mip Level
                TEX_FORMAT.to_sized_internal_format(),
                256,
                3,
            );
        }

        Ok(Self {
            name: config.name.clone(),
            dirty: false,
            texture: new_tex,
        })
    }
}

impl Node for Keyboard {
    fn get_name(&self) -> &String {
        return &self.name;
    }

    fn update_resolution(&mut self, _gl: &glow::Context, _screen_resolution: &[i32; 2]) {}

    fn bind(&mut self, gl: &glow::Context, _quad: &Quad, game_state: &GameState) {
        unsafe {
            if game_state.keys_dirty {
                gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));

                let key_state_array =
                    std::mem::transmute::<&[i8; 768], &[u8; 768]>(&game_state.keys);

                gl.tex_sub_image_2d(
                    glow::TEXTURE_2D,
                    0,   // MipLevel
                    0,   // X Offs
                    0,   // Y Offs
                    256, // Width
                    3,   // Height
                    TEX_FORMAT.to_format(),
                    TEX_FORMAT.to_type(),
                    glow::PixelUnpackData::Slice(key_state_array),
                );
                self.dirty = true;
            }
        }
    }

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

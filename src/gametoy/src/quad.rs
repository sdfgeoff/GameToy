/// LOOK AT:
/// https://rust-tutorials.github.io/learn-opengl/basics/001-drawing-a-triangle.html
use glow::{Buffer, Context, HasContext, ARRAY_BUFFER, FLOAT, STATIC_DRAW};

/// An error with this whole object.
#[derive(Debug)]
pub enum QuadError {
    /// Failed to upload buffer data to the GPU
    BufferCreationFailed(String),
}

pub struct Quad {
    position_buffer: Buffer,
}

impl Quad {
    pub fn new(gl: &Context) -> Result<Self, QuadError> {
        let position_buffer =
            unsafe { upload_array_f32(gl, vec![-1.0, 1.0, 1.0, 1.0, 0.0, -1.0])? };
        Ok(Self { position_buffer })
    }

    pub fn bind(&mut self, gl: &Context) {
        unsafe {
            gl.bind_buffer(ARRAY_BUFFER, Some(self.position_buffer));

            gl.vertex_attrib_pointer_f32(
                0,     //index: u32,
                2,     //size: i32,
                FLOAT, //data_type: u32,
                false, //normalized: bool,
                0,     //(core::mem::size_of::<f32>() * 2) as i32, //stride: i32,
                0,     //offset: i32
            );

            gl.enable_vertex_attrib_array(0);
        }
    }
}

unsafe fn upload_array_f32(gl: &Context, vertices: Vec<f32>) -> Result<Buffer, QuadError> {
    let vao = gl
        .create_vertex_array()
        .map_err(QuadError::BufferCreationFailed)?;
    gl.bind_vertex_array(Some(vao));
    let vbo = gl
        .create_buffer()
        .map_err(QuadError::BufferCreationFailed)?;
    gl.bind_buffer(ARRAY_BUFFER, Some(vbo));

    gl.buffer_data_u8_slice(ARRAY_BUFFER, as_u8_slice(&vertices), STATIC_DRAW);

    Ok(vbo)
}

fn as_u8_slice(v: &[f32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<i32>(),
        )
    }
}

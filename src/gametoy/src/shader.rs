use glow::{Context, HasContext, Program, Shader, FRAGMENT_SHADER, VERTEX_SHADER};

#[derive(Debug)]
pub enum ShaderError {
    ShaderAllocError(String),
    ShaderProgramAllocError(String),
    ShaderCompileError {
        shader_type: u32,
        compiler_output: String,
    },
    ShaderLinkError(String),
}

pub struct SimpleShader {
    pub program: Program,
    //pub attrib_vertex_positions: u32,
}

impl SimpleShader {
    pub fn new(gl: &Context, vert: &str, frag: &str) -> Result<Self, ShaderError> {
        let program = unsafe { init_shader_program(gl, vert, frag)? };

        //let attrib_vertex_positions = gl.get_attrib_location(program, "aVertexPosition");

        Ok(Self {
            program,
            //attrib_vertex_positions: attrib_vertex_positions as u32,
        })
    }

    pub fn bind(&mut self, gl: &Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
}

unsafe fn load_shader(
    gl: &Context,
    shader_type: u32,
    shader_text: &str,
) -> Result<Shader, ShaderError> {
    let shader = gl
        .create_shader(shader_type)
        .map_err(ShaderError::ShaderAllocError)?;

    gl.shader_source(shader, shader_text);
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        let compiler_output = gl.get_shader_info_log(shader);
        gl.delete_shader(shader);
        return Err(ShaderError::ShaderCompileError {
            shader_type,
            compiler_output: compiler_output,
        });
    }
    Ok(shader)
}

pub unsafe fn init_shader_program(
    gl: &Context,
    vert_source: &str,
    frag_source: &str,
) -> Result<Program, ShaderError> {
    let vert_shader = load_shader(gl, VERTEX_SHADER, vert_source)?;
    let frag_shader = load_shader(gl, FRAGMENT_SHADER, frag_source)?;

    let shader_program = gl
        .create_program()
        .map_err(ShaderError::ShaderProgramAllocError)?;
    gl.attach_shader(shader_program, vert_shader);
    gl.attach_shader(shader_program, frag_shader);

    gl.link_program(shader_program);

    if !(gl.get_program_link_status(shader_program)) {
        let compiler_output = gl.get_program_info_log(shader_program);
        gl.delete_program(shader_program);
        gl.delete_shader(vert_shader);
        gl.delete_shader(frag_shader);
        return Err(ShaderError::ShaderLinkError(compiler_output));
    }

    Ok(shader_program)
}

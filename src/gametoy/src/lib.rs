pub use glow;
pub use tar;
use glow::HasContext;

mod quad;
mod shader;
mod config_file;


pub struct GameToy<R: std::io::Read> {
    gl: glow::Context,
    data: tar::Archive<R>,
    
    // Time the last frame was rendered - used to calculate dt
    prev_render_time: f64,
    // Monotonic non-decreasing clock
    time_since_start: f64,

    // Everything is rendered on the same quad, so lets just chuck that here
    quad: quad::Quad,

    shader_program: shader::SimpleShader

}


impl<R: std::io::Read> GameToy<R> {
    pub fn new(gl: glow::Context, data: tar::Archive<R>) -> Self {

        let quad = quad::Quad::new(&gl).expect("Failed to create quad");

        let shader_program = shader::SimpleShader::new(
                &gl, 
                include_str!("resources/shader.vert"), 
                include_str!("resources/shader.frag")
            ).unwrap();
        
        unsafe {
            gl.clear_color(0.0, 1.0, 1.0, 1.0);
        }

        Self {
            gl,
            data,
            prev_render_time: 0.0,
            time_since_start: 0.0,
            quad,
            shader_program
        }
    }

    // Perform a complete render
    // Requires the time as seconds past the unix epoch. Note that
    // if you pass this in as zero, the simulation will assume a frametime of
    // 60FPS.
    pub fn render(&mut self, time_since_unix_epoch: f64) {
        let dt = if self.prev_render_time == 0.0 {
            0.016
        } else {
            // Cap the dt at 0.0 - time should not move backwards
            f64::max(time_since_unix_epoch - self.prev_render_time, 0.0)
        };

        self.prev_render_time = time_since_unix_epoch;
        self.time_since_start += dt;

        unsafe{
            self.gl.clear(glow::COLOR_BUFFER_BIT);

            self.shader_program.bind(&self.gl);
            self.quad.bind(&self.gl);
            
            self.gl.draw_arrays(glow::TRIANGLES, 0, 3);

        }
    }

    // Sets the size to render at
    pub fn resize(&mut self, x_pixels: u32, y_pixels: u32) {
        println!("[OK] Resizing to: {}, {}", x_pixels, y_pixels);
        unsafe {
            self.gl.viewport(0, 0, x_pixels as i32, y_pixels as i32);
        }
    }
    
    /*
    fn destroy() {

    }

    fn set_key_state(keystate) {

    }

    fn set_mouse_state(mousestate) {

    }
    */
}
pub use glow;
pub use tar;
use glow::HasContext;
use std::collections::HashMap;
use std::io::Read;
use serde_json;

mod quad;
mod shader;
mod config_file;

#[derive(Debug)]
enum GameDataError {
    TarError(std::io::Error),
    ConfigFileParseError(serde_json::Error),
    MissingConfigFile,
}

struct GameData {
    config_file: config_file::ConfigFile,
    textures:HashMap<String,Vec<u8>>,
    shader_sources:HashMap<String,String>,
}

impl GameData {
    fn from_tar<R> (mut data: tar::Archive<R>) -> Result<Self, GameDataError> 
    where R: Read
    
    {
        let mut config_file: Option<config_file::ConfigFile> = None;
        let mut textures = HashMap::new();
        let mut shader_sources = HashMap::new();


        let entries = data.entries().map_err(GameDataError::TarError)?;
        for file in entries {
            // Make sure there wasn't an I/O error
            let mut file = file.map_err(GameDataError::TarError)?;
    
            // Inspect metadata about the file
            let filename = file.header().path().map_err(GameDataError::TarError)?;
            let filename_str = filename.to_str().unwrap();
            println!("Loading {:?}", filename); 
    
            if filename_str == "data.json" {
                //config_file = ConfigFile::
                // files implement the Read trait
                //let mut s = String::new();
                //file.read_to_string(&mut s).unwrap();
                
                config_file = Some(
                    serde_json::from_reader(file).map_err(GameDataError::ConfigFileParseError)?
                );
            }
        }

        let config_file = config_file.ok_or(GameDataError::MissingConfigFile)?;
        
        Ok(Self {
            config_file,
            textures,
            shader_sources,
        })
        
    }
}
    


pub struct GameToy {
    gl: glow::Context,
    game_data: GameData,
    
    // Time the last frame was rendered - used to calculate dt
    prev_render_time: f64,
    // Monotonic non-decreasing clock
    time_since_start: f64,

    // Everything is rendered on the same quad, so lets just chuck that here
    quad: quad::Quad,


    shader_program: shader::SimpleShader

}


impl GameToy {
    pub fn new<R>(gl: glow::Context, mut data: tar::Archive<R>) -> Self where R: Read  {

        let game_data = GameData::from_tar(data).expect("Game Data Error");

        let quad = quad::Quad::new(&gl).expect("Failed to create quad");

        let shader_program = shader::SimpleShader::new(
                &gl, 
                include_str!("resources/shader.vert"), 
                include_str!("resources/shader.frag")
            ).expect("Failed to create simple shader");
        
        unsafe {
            gl.clear_color(0.0, 1.0, 1.0, 1.0);
        }

        Self {
            gl,
            game_data,
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
            self.quad.bind(&self.gl);

            self.shader_program.bind(&self.gl);
            
            
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
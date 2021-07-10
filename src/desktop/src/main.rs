use gametoy;
use gametoy::glow;
use gametoy::tar;
use std::fs;
use std::env;


use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

const TAR_FILE: &'static str = "datapack.tar";
const DATA_FOLDER: &'static str = "data";



fn main() {
    // Attempt to read the data package
    let tar = load_tar().expect("Unable to load TAR");

    // Create our window
    let (gl, shader_version, window, event_loop) = {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("GameToy: loading")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        
        
        let window = unsafe {
            glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap()
            };
        let gl = unsafe {
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _)
        };
        (gl, "#version 410", window, event_loop)
    };

    
    let mut toy = gametoy::GameToy::new(gl, tar);
    

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {
                return;
            }
            Event::MainEventsCleared => {
                window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Put rendering in here apparently

                let since_the_epoch = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time is pre 1970???");

                toy.render(since_the_epoch.as_secs_f64());
                window.swap_buffers().unwrap();
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {

                    toy.resize(physical_size.width, physical_size.height);

                    window.resize(*physical_size);
                }
                WindowEvent::CloseRequested => {
                    // Put something in here too
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        }
    });
}



fn load_tar() -> Option<tar::Archive<fs::File>> {

    let exe_path = env::current_exe().expect("Failed to determine executable location");
    
    let mut exe_dir = exe_path.clone();
    exe_dir.pop();
    let mut data_folder = exe_dir.clone();
    data_folder.push(DATA_FOLDER);
    let mut tar_file_path = exe_dir.clone();
    tar_file_path.push(TAR_FILE);

    // If there is a folder for data, assemble it into a package and run
    // the game with that.
    if let Ok(entries) = fs::read_dir(&data_folder) {

        println!("[OK] Found data directory. Creating Bundle");
        let file = fs::File::create(tar_file_path.clone()).unwrap();
        let mut a = tar::Builder::new(file);

        for entry in entries {
            let entry = entry.expect("Failed to read directory");
            let path = entry.path();

            println!("[OK] Bundling file: {:?}", &path);
            a.append_file(
                path.file_name().expect("No filename???"), 
                &mut fs::File::open(path.clone()).expect("Failed to open file for packing")
            ).unwrap();
        }
    } else {
        println!("[OK] No data directory: {:?}", data_folder);
    }


    if let Ok(file) = fs::File::open(tar_file_path.clone()) {
        println!("[OK] Running with package: {:?}", tar_file_path);
        return Some(tar::Archive::new(file))
    } else {
        println!("[WRN] No tar bundle found at path {:?}", tar_file_path);
    }

    None
}
use gametoy;
use gametoy::glow;
use gametoy::tar;
use std::env;
use std::fs;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

const TAR_FILE: &'static str = "datapack.tar";
const DATA_FOLDER: &'static str = "data";

fn main() {
    // Attempt to read the data package
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let tar = load_tar(args).expect("Unable to load TAR");

    // Create our window
    let (gl, window, event_loop) = {
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
        (gl, window, event_loop)
    };

    let mut toy = match gametoy::GameToy::new(&gl, tar, true) {
        Ok(toy) => toy,
        Err(gametoy::GameToyError::NodeCreateError(
            nodename,
            gametoy::nodes::NodeError::ShaderError(
                gametoy::shader::ShaderError::ShaderCompileError {
                    shader_type: _,
                    compiler_output,
                    shader_text,
                },
            ),
        )) => {
            let lines = shader_text.split('\n');
            for (line_id, line_text) in lines.enumerate() {
                println!("{:4} | {}", line_id + 1, line_text);
            }

            println!(
                "Error creating node: \"{}\"\n\n{}",
                nodename, compiler_output
            );
            return;
        }
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

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

                toy.render(&gl, since_the_epoch.as_secs_f64())
                    .expect("Failed to render");
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
                WindowEvent::KeyboardInput { ref input, .. } => {
                    if let Some(meaning) = input.virtual_keycode {
                        let keycode = to_keycode(meaning);
                        if let Some(code) = keycode {
                            toy.set_key_state(
                                code,
                                input.state == glutin::event::ElementState::Pressed,
                            );
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}

fn load_tar(args: Vec<String>) -> Option<tar::Archive<fs::File>> {
    let exe_path = env::current_exe().expect("Failed to determine executable location");

    let mut exe_dir = exe_path.clone();
    exe_dir.pop();

    let data_folder = {
        if args.len() == 2 {
            let mut data_folder = env::current_dir().expect("Unable to determine CWD").clone();
            data_folder.push(args[1].clone());
            println!("[OK] Using override data directory: {:?}", data_folder);
            data_folder
        } else {
            let mut data_folder = exe_dir.clone();
            data_folder.push(DATA_FOLDER);
            println!("[OK] Using default data directory: {:?}", data_folder);
            data_folder
        }
    };

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
            let metadata = fs::metadata(&path).expect("File Deleted while bundling");
            if metadata.is_file() {
                println!("[OK] Bundling file: {:?}", &path);
                a.append_file(
                    path.file_name().expect("No filename???"),
                    &mut fs::File::open(path.clone()).expect("Failed to open file for packing"),
                )
                .unwrap();
            }
        }
    } else {
        println!("[OK] No data directory: {:?}", data_folder);
    }

    if let Ok(file) = fs::File::open(tar_file_path.clone()) {
        println!("[OK] Running with package: {:?}", tar_file_path);
        return Some(tar::Archive::new(file));
    } else {
        println!("[WRN] No tar bundle found at path {:?}", tar_file_path);
    }

    None
}

fn to_keycode(key: glutin::event::VirtualKeyCode) -> Option<u32> {
    match key {
        glutin::event::VirtualKeyCode::Key1 => Some(49),
        glutin::event::VirtualKeyCode::Key2 => Some(50),
        glutin::event::VirtualKeyCode::Key3 => Some(51),
        glutin::event::VirtualKeyCode::Key4 => Some(52),
        glutin::event::VirtualKeyCode::Key5 => Some(53),
        glutin::event::VirtualKeyCode::Key6 => Some(54),
        glutin::event::VirtualKeyCode::Key7 => Some(55),
        glutin::event::VirtualKeyCode::Key8 => Some(56),
        glutin::event::VirtualKeyCode::Key9 => Some(57),
        glutin::event::VirtualKeyCode::Key0 => Some(48),
        glutin::event::VirtualKeyCode::A => Some(65),
        glutin::event::VirtualKeyCode::B => Some(66),
        glutin::event::VirtualKeyCode::C => Some(67),
        glutin::event::VirtualKeyCode::D => Some(68),
        glutin::event::VirtualKeyCode::E => Some(69),
        glutin::event::VirtualKeyCode::F => Some(70),
        glutin::event::VirtualKeyCode::G => Some(71),
        glutin::event::VirtualKeyCode::H => Some(72),
        glutin::event::VirtualKeyCode::I => Some(73),
        glutin::event::VirtualKeyCode::J => Some(74),
        glutin::event::VirtualKeyCode::K => Some(75),
        glutin::event::VirtualKeyCode::L => Some(76),
        glutin::event::VirtualKeyCode::M => Some(77),
        glutin::event::VirtualKeyCode::N => Some(78),
        glutin::event::VirtualKeyCode::O => Some(79),
        glutin::event::VirtualKeyCode::P => Some(80),
        glutin::event::VirtualKeyCode::Q => Some(81),
        glutin::event::VirtualKeyCode::R => Some(82),
        glutin::event::VirtualKeyCode::S => Some(83),
        glutin::event::VirtualKeyCode::T => Some(84),
        glutin::event::VirtualKeyCode::U => Some(85),
        glutin::event::VirtualKeyCode::V => Some(86),
        glutin::event::VirtualKeyCode::W => Some(87),
        glutin::event::VirtualKeyCode::X => Some(88),
        glutin::event::VirtualKeyCode::Y => Some(89),
        glutin::event::VirtualKeyCode::Z => Some(80),
        glutin::event::VirtualKeyCode::Escape => Some(27),
        glutin::event::VirtualKeyCode::F1 => Some(112),
        glutin::event::VirtualKeyCode::F2 => Some(113),
        glutin::event::VirtualKeyCode::F3 => Some(114),
        glutin::event::VirtualKeyCode::F4 => Some(115),
        glutin::event::VirtualKeyCode::F5 => Some(116),
        glutin::event::VirtualKeyCode::F6 => Some(117),
        glutin::event::VirtualKeyCode::F7 => Some(118),
        glutin::event::VirtualKeyCode::F8 => Some(119),
        glutin::event::VirtualKeyCode::F9 => Some(120),
        glutin::event::VirtualKeyCode::F10 => Some(121),
        glutin::event::VirtualKeyCode::F11 => Some(122),
        glutin::event::VirtualKeyCode::F12 => Some(123),
        glutin::event::VirtualKeyCode::F13 => None,
        glutin::event::VirtualKeyCode::F14 => None,
        glutin::event::VirtualKeyCode::F15 => None,
        glutin::event::VirtualKeyCode::F16 => None,
        glutin::event::VirtualKeyCode::F17 => None,
        glutin::event::VirtualKeyCode::F18 => None,
        glutin::event::VirtualKeyCode::F19 => None,
        glutin::event::VirtualKeyCode::F20 => None,
        glutin::event::VirtualKeyCode::F21 => None,
        glutin::event::VirtualKeyCode::F22 => None,
        glutin::event::VirtualKeyCode::F23 => None,
        glutin::event::VirtualKeyCode::F24 => None,
        glutin::event::VirtualKeyCode::Snapshot => Some(44),
        glutin::event::VirtualKeyCode::Scroll => Some(145),
        glutin::event::VirtualKeyCode::Pause => Some(19),
        glutin::event::VirtualKeyCode::Insert => Some(45),
        glutin::event::VirtualKeyCode::Home => Some(36),
        glutin::event::VirtualKeyCode::Delete => Some(46),
        glutin::event::VirtualKeyCode::End => Some(35),
        glutin::event::VirtualKeyCode::PageDown => Some(34),
        glutin::event::VirtualKeyCode::PageUp => Some(33),
        glutin::event::VirtualKeyCode::Left => Some(37),
        glutin::event::VirtualKeyCode::Up => Some(38),
        glutin::event::VirtualKeyCode::Right => Some(39),
        glutin::event::VirtualKeyCode::Down => Some(40),
        glutin::event::VirtualKeyCode::Back => Some(8),
        glutin::event::VirtualKeyCode::Return => Some(13),
        glutin::event::VirtualKeyCode::Space => Some(32),
        glutin::event::VirtualKeyCode::Compose => None,
        glutin::event::VirtualKeyCode::Caret => None, // Unsure
        glutin::event::VirtualKeyCode::Numlock => Some(144),
        glutin::event::VirtualKeyCode::Numpad0 => Some(96),
        glutin::event::VirtualKeyCode::Numpad1 => Some(97),
        glutin::event::VirtualKeyCode::Numpad2 => Some(98),
        glutin::event::VirtualKeyCode::Numpad3 => Some(99),
        glutin::event::VirtualKeyCode::Numpad4 => Some(100),
        glutin::event::VirtualKeyCode::Numpad5 => Some(101),
        glutin::event::VirtualKeyCode::Numpad6 => Some(102),
        glutin::event::VirtualKeyCode::Numpad7 => Some(103),
        glutin::event::VirtualKeyCode::Numpad8 => Some(104),
        glutin::event::VirtualKeyCode::Numpad9 => Some(105),
        glutin::event::VirtualKeyCode::NumpadAdd => Some(107),
        glutin::event::VirtualKeyCode::NumpadDivide => Some(111),
        glutin::event::VirtualKeyCode::NumpadDecimal => Some(110),
        glutin::event::VirtualKeyCode::NumpadComma => Some(188), // Same as reg. comma
        glutin::event::VirtualKeyCode::NumpadEnter => Some(13),  // Same as reg. enter
        glutin::event::VirtualKeyCode::NumpadEquals => Some(187), // Same as reg. equals
        glutin::event::VirtualKeyCode::NumpadMultiply => Some(106),
        glutin::event::VirtualKeyCode::NumpadSubtract => Some(109),
        glutin::event::VirtualKeyCode::AbntC1 => None,
        glutin::event::VirtualKeyCode::AbntC2 => None,
        glutin::event::VirtualKeyCode::Apostrophe => Some(222),
        glutin::event::VirtualKeyCode::Apps => None,
        glutin::event::VirtualKeyCode::Asterisk => None,
        glutin::event::VirtualKeyCode::At => None,
        glutin::event::VirtualKeyCode::Ax => None,
        glutin::event::VirtualKeyCode::Backslash => Some(220),
        glutin::event::VirtualKeyCode::Calculator => Some(183),
        glutin::event::VirtualKeyCode::Capital => Some(20),
        glutin::event::VirtualKeyCode::Colon => Some(186),
        glutin::event::VirtualKeyCode::Comma => Some(188),
        glutin::event::VirtualKeyCode::Convert => None,
        glutin::event::VirtualKeyCode::Equals => Some(187),
        glutin::event::VirtualKeyCode::Grave => Some(192),
        glutin::event::VirtualKeyCode::Kana => None,
        glutin::event::VirtualKeyCode::Kanji => None,
        glutin::event::VirtualKeyCode::LAlt => Some(18),
        glutin::event::VirtualKeyCode::LBracket => Some(219),
        glutin::event::VirtualKeyCode::LControl => Some(17),
        glutin::event::VirtualKeyCode::LShift => Some(16),
        glutin::event::VirtualKeyCode::LWin => Some(91),
        glutin::event::VirtualKeyCode::Mail => None,
        glutin::event::VirtualKeyCode::MediaSelect => None,
        glutin::event::VirtualKeyCode::MediaStop => None,
        glutin::event::VirtualKeyCode::Minus => Some(189),
        glutin::event::VirtualKeyCode::Mute => None,
        glutin::event::VirtualKeyCode::MyComputer => Some(182),
        glutin::event::VirtualKeyCode::NavigateForward => None,
        glutin::event::VirtualKeyCode::NavigateBackward => None,
        glutin::event::VirtualKeyCode::NextTrack => None,
        glutin::event::VirtualKeyCode::NoConvert => None,
        glutin::event::VirtualKeyCode::OEM102 => Some(220), // Unsure
        glutin::event::VirtualKeyCode::Period => Some(190),
        glutin::event::VirtualKeyCode::PlayPause => None,
        glutin::event::VirtualKeyCode::Plus => Some(107),
        glutin::event::VirtualKeyCode::Power => None,
        glutin::event::VirtualKeyCode::PrevTrack => None,
        glutin::event::VirtualKeyCode::RAlt => Some(18),
        glutin::event::VirtualKeyCode::RBracket => Some(221),
        glutin::event::VirtualKeyCode::RControl => Some(17),
        glutin::event::VirtualKeyCode::RShift => Some(16),
        glutin::event::VirtualKeyCode::RWin => Some(92),
        glutin::event::VirtualKeyCode::Semicolon => Some(186),
        glutin::event::VirtualKeyCode::Slash => Some(191),
        glutin::event::VirtualKeyCode::Sleep => None,
        glutin::event::VirtualKeyCode::Stop => None,
        glutin::event::VirtualKeyCode::Sysrq => None, // Unsure
        glutin::event::VirtualKeyCode::Tab => Some(9),
        glutin::event::VirtualKeyCode::Underline => None,
        glutin::event::VirtualKeyCode::Unlabeled => None,
        glutin::event::VirtualKeyCode::VolumeDown => None,
        glutin::event::VirtualKeyCode::VolumeUp => None,
        glutin::event::VirtualKeyCode::Wake => None,
        glutin::event::VirtualKeyCode::WebBack => None,
        glutin::event::VirtualKeyCode::WebFavorites => None,
        glutin::event::VirtualKeyCode::WebForward => None,
        glutin::event::VirtualKeyCode::WebHome => None,
        glutin::event::VirtualKeyCode::WebRefresh => None,
        glutin::event::VirtualKeyCode::WebSearch => None,
        glutin::event::VirtualKeyCode::WebStop => None,
        glutin::event::VirtualKeyCode::Yen => None,
        glutin::event::VirtualKeyCode::Copy => None,
        glutin::event::VirtualKeyCode::Paste => None,
        glutin::event::VirtualKeyCode::Cut => None,
    }
}

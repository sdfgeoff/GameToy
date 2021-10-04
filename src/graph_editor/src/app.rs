use eframe::{egui, epi};
use gametoy::gamedata::CONFIG_FILE_NAME;
use rfd::FileDialog;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use super::metadata;
use super::nodes;
use super::render_order;
use super::state;

use super::state::{StateOperation, UiLayoutMode};

pub struct GametoyGraphEditor {
    pub state: state::EditorState,
    pub reactor: state::Reactor,
    pub dirty: bool,
}

impl Default for GametoyGraphEditor {
    fn default() -> Self {
        Self {
            state: state::templates::simple_project(),
            reactor: state::Reactor::new(),
            dirty: false,
        }
    }
}

impl GametoyGraphEditor {
    fn open_file(&mut self) {
        if self.dirty {
            println!("Not loading file: project is dirty");
            return;
        }

        let exe_path = env::current_exe().expect("Failed to determine executable location");

        let mut exe_dir = exe_path.clone();
        exe_dir.pop();

        let mut dialog = FileDialog::new();
        dialog = dialog.add_filter("json", &["json"]).set_directory(exe_dir);

        let project_path = dialog.pick_file();
        if let Some(filepath) = project_path {
            match load_data_file(&filepath) {
                Ok(conf) => {
                    self.reactor
                        .queue_operation(state::StateOperation::SetProjectPath(Some(filepath)));
                    self.reactor
                        .queue_operation(state::StateOperation::LoadFromConfigFile(conf));
                    self.dirty = false; // We just opened the file. Nothings changed yet.
                }
                Err(err) => {
                    println!("Loading data file failed: {:?}", err)
                }
            }
        } else {
            println!("No filepath to load");
        }
    }
    /// Save the project to it's current project location
    fn save_file(&self, filepath: &PathBuf) {
        if let Err(err) = save_data_file(&self.state.project_data.config_file, filepath) {
            println!("Saving data file failed: {:?}", err)
        }
    }
    /// Open a file dialog and then save the file in that location
    fn save_as(&mut self) {
        let mut dialog = FileDialog::new();

        let start_folder = match &self.state.project_file {
            Some(file) => {
                let mut file_path = file.clone();
                file_path.pop();
                file_path
            }
            None => {
                let mut exe_path =
                    env::current_exe().expect("Failed to determine executable location");
                exe_path.pop();
                exe_path
            }
        };

        dialog = dialog
            .add_filter("json", &["json"])
            .set_file_name(CONFIG_FILE_NAME)
            .set_directory(start_folder);
        if let Some(output_file) = dialog.save_file() {
            self.save_file(&output_file);
            self.reactor
                .queue_operation(state::StateOperation::SetProjectPath(Some(output_file)));
            self.dirty = false
        }
    }

    fn new(&mut self) {
        if !self.dirty {
            self.state = state::templates::simple_project();
            self.dirty = false; // No point forcing a save of a default project
        } else {
            println!("Not creating new: project is dirty")
        }
    }
}

// Read a config file from disk
fn load_data_file(pathbuf: &PathBuf) -> Result<gametoy::config_file::ConfigFile, Box<dyn Error>> {
    let data_file = File::open(pathbuf)?;
    Ok(serde_json::from_reader(data_file)?)
}

// Write a config file to disk
fn save_data_file(
    data: &gametoy::config_file::ConfigFile,
    pathbuf: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(pathbuf)?;
    serde_json::to_writer(data_file, data)?;
    Ok(())
}

impl epi::App for GametoyGraphEditor {
    fn name(&self) -> &str {
        "Gametoy"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>, gl: &glow::Context) {
        //let mut new_proj = self.state.project_data.clone();
        ctx.set_pixels_per_point(1.2);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    let new_button = egui::Button::new("New").enabled(!self.dirty);
                    if ui.add(new_button).clicked() {
                        self.new();
                    }
                    let open_button = egui::Button::new("Open").enabled(!self.dirty);
                    if ui.add(open_button).clicked() {
                        self.open_file();
                    }
                    ui.separator();
                    let save_button =
                        egui::Button::new("Save").enabled(self.state.project_file.is_some());
                    if ui.add(save_button).clicked() {
                        if let Some(path) = &self.state.project_file {
                            self.save_file(path);
                            self.dirty = false;
                        }
                    }
                    if ui.button("Save As").clicked() {
                        self.save_as();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("left_side_panel").show(ctx, |ui| {
            let scroll_area = egui::ScrollArea::auto_sized();

            scroll_area.show(ui, |ui| {
                egui::CollapsingHeader::new("Metadata")
                    .default_open(true)
                    .show(ui, |ui| {
                        metadata::draw_metadata(
                            ui,
                            &self.state.project_data.config_file.metadata,
                            &mut self.reactor,
                        );
                        ui.separator();
                    });

                egui::CollapsingHeader::new("Render Order")
                    .default_open(true)
                    .show(ui, |ui| {
                        render_order::render_order_widget(
                            ui,
                            &mut self.reactor,
                            &self.state.project_data.config_file.graph.nodes,
                        );
                        ui.separator();
                    });

                egui::CollapsingHeader::new("Node Properties")
                    .default_open(true)
                    .show(ui, |ui| {
                        match self.state.ui_state.selected_node_id {
                            Some(id) => {
                                match self.state.project_data.config_file.graph.nodes.get(id) {
                                    Some(node) => {
                                        nodes::draw_node_properties(
                                            ui,
                                            &mut self.reactor,
                                            node,
                                            id,
                                        );
                                    }
                                    None => {}
                                }
                            }
                            None => {
                                ui.label("Select a Node");
                            }
                        };
                        ui.separator();
                    })
            });
        });

        let mut outp_tex = None;

        if let Some(gametoy) = &mut self.state.gametoy_instance {
            match gametoy {
                Ok(gametoy) => {
                    gametoy.render(gl, 0.0);
                    use std::any::Any;

                    if let Some(output_ref) = &gametoy.output_node_maybe {
                        let input_tex = output_ref.borrow().get_input_texture(
                            &gametoy::nodes::Output::INPUT_BUFFER_NAME.to_string(),
                        );
                        if let Ok(tex) = input_tex {
                            outp_tex = tex.clone();
                        }
                    }
                    unsafe {
                        use glow::HasContext;
                        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        } else {
            self.reactor.queue_operation(StateOperation::CompileGametoy);
        }

        egui::SidePanel::right("right_side_panel")
            .default_width(300.0)
            .show(ctx, |ui| {
                // Top is taken up by an image
                let scroll_area = egui::ScrollArea::auto_sized();
                scroll_area.show(ui, |ui| {
                    let available_space = ui.available_size();
                    let render_size = [
                        (available_space.x) as u32,
                        (available_space.x * 9.0 / 16.0) as u32,
                    ];

                    if render_size != self.state.game_play_state.render_size {
                        self.reactor
                            .queue_operation(StateOperation::SetGameRenderSize(render_size));
                    }
                    let texture_id = match outp_tex {
                        Some(tex) => {
                            // ctx.debug_painter().register_glow_texture(tex)
                            egui::TextureId::Egui
                        }
                        None => egui::TextureId::Egui,
                    };

                    ui.add(egui::Image::new(
                        texture_id,
                        [render_size[0] as f32, render_size[1] as f32],
                    ));

                    ui.horizontal(|ui| {
                        ui.label(format!("{} x {}", render_size[0], render_size[1]));
                    });

                    ui.separator();
                    ui.heading("Project Files:");
                    egui::Grid::new("project_file_grid")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            for filename in self.state.project_data.files.keys() {
                                if ui.button(filename.to_string()).clicked() {
                                    self.reactor
                                        .queue_operation(StateOperation::SetUiLayoutMode(
                                            UiLayoutMode::TextEditor(filename.to_string()),
                                        ));
                                };
                                ui.end_row();
                            }
                            if ui.button("data.json").clicked() {
                                self.reactor
                                    .queue_operation(StateOperation::SetUiLayoutMode(
                                        UiLayoutMode::GraphEditor,
                                    ));
                            }
                        });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            match &self.state.ui_state.ui_layout_mode {
                UiLayoutMode::GraphEditor => {
                    super::graph::draw_rendergraph_editor(
                        ui,
                        &mut self.reactor,
                        &mut self.state.ui_state.node_context,
                        &self.state.project_data.config_file,
                    );
                }
                UiLayoutMode::TextEditor(filename) => {
                    match self.state.project_data.files.get(filename) {
                        Some(buffer) => match String::from_utf8(buffer.clone()) {
                            Ok(mut buffer) => {
                                let scroll_area = egui::ScrollArea::auto_sized();
                                let orig = buffer.clone();
                                scroll_area.show(ui, |ui| {
                                    let size = ui.available_size();
                                    ui.add_sized(
                                        size,
                                        egui::TextEdit::multiline(&mut buffer)
                                            .code_editor()
                                            .id_source(filename)
                                            .desired_width(size.x),
                                    );
                                });
                                if buffer != orig {
                                    self.reactor.queue_operation(StateOperation::WriteToFile(
                                        filename.clone(),
                                        buffer.into_bytes(),
                                    ));
                                }
                            }
                            Err(err) => {
                                ui.label(format!("File not readable as utf-8: {}", err));
                            }
                        },
                        None => {
                            ui.label("No File Selected");
                        }
                    }
                }
            }
        });

        self.reactor
            .queue_operation(StateOperation::RemoveInvalidLinks);
        let old_project_state = self.state.project_data.clone();
        self.reactor.react(&mut self.state, gl);
        if old_project_state != self.state.project_data {
            self.dirty = true
        }
    }
}

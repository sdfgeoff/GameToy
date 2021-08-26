use eframe::{egui, epi};
use gametoy::gamedata::CONFIG_FILE_NAME;
use rfd::FileDialog;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use super::graph;
use super::helpers;
use super::metadata;

pub struct GametoyGraphEditor {
    project_file: Option<PathBuf>,
    project_data: gametoy::config_file::ConfigFile,
    dirty: bool,
    selected_node_id: Option<usize>,
}

fn create_default_project() -> gametoy::config_file::ConfigFile {
    gametoy::config_file::ConfigFile {
        metadata: gametoy::config_file::MetaData {
            game_name: "Your Awesome Game".to_string(),
            game_version: "0.0.0".to_string(),
            release_date: "Today".to_string(),
            website: "".to_string(),
            author_name: "You".to_string(),
            license: "CC-BY-SA-NC 3.0".to_string(),
        },
        graph: gametoy::config_file::GraphConfig {
            nodes: vec![
                gametoy::config_file::Node::Keyboard(gametoy::config_file::KeyboardConfig {
                    name: "Keyboard".to_string(),
                }),
                gametoy::config_file::Node::RenderPass(gametoy::config_file::RenderPassConfig {
                    name: "Render Pass 1".to_string(),
                    output_texture_slots: vec![gametoy::config_file::OutputBufferConfig {
                        name: "RenderOut".to_string(),
                        format: gametoy::config_file::OutputBufferFormat::RGB8,
                    }],
                    input_texture_slots: vec![gametoy::config_file::InputBufferConfig {
                        name: "KeyboardInput".to_string(),
                    }],
                    resolution_scaling_mode:
                        gametoy::config_file::ResolutionScalingMode::ViewportScale(1.0, 1.0),
                    fragment_shader_paths: vec![],
                    execution_mode: gametoy::config_file::ExecutionMode::Always,
                }),
                gametoy::config_file::Node::Output(gametoy::config_file::OutputConfig {
                    name: "Output".to_string(),
                }),
            ],
            links: vec![],
        },
    }
}

impl Default for GametoyGraphEditor {
    fn default() -> Self {
        Self {
            project_file: None,
            project_data: create_default_project(),
            dirty: false,
            selected_node_id: None,
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
        self.project_file = dialog.pick_file();

        if let Some(filepath) = &self.project_file {
            match load_data_file(filepath) {
                Ok(conf) => {
                    self.project_data = conf;
                    self.dirty = false; // We just opened the file. Nothings changed yet.
                }
                Err(err) => {
                    println!("Loading data file failed: {:?}", err)
                }
            }
        }
    }
    /// Save the project to it's current project location
    fn save(&mut self) {
        if let Some(proj_file) = &self.project_file {
            if let Err(err) = save_data_file(&self.project_data, proj_file) {
                println!("Saving data file failed: {:?}", err)
            }
        }
    }
    /// Open a file dialog and then save the file in that location
    fn save_as(&mut self) {
        let mut dialog = FileDialog::new();

        let start_folder = match &self.project_file {
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
        self.project_file = dialog.save_file();
        self.save();
    }

    fn new(&mut self) {
        if !self.dirty {
            self.project_data = create_default_project();
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
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let mut new_proj = self.project_data.clone();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    let new_button = egui::Button::new("New").enabled(!self.dirty);
                    if ui.add(new_button).clicked() {
                        self.new();
                        new_proj = self.project_data.clone();
                    }
                    let open_button = egui::Button::new("Open").enabled(!self.dirty);
                    if ui.add(open_button).clicked() {
                        self.open_file();
                        new_proj = self.project_data.clone();
                    }
                    ui.separator();
                    let save_button =
                        egui::Button::new("Save").enabled(self.project_file.is_some());
                    if ui.add(save_button).clicked() {
                        self.save();
                    }
                    if ui.button("Save As").clicked() {
                        self.save_as();
                        new_proj = self.project_data.clone();
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
                        metadata::draw_metadata(&mut new_proj, ui);
                        ui.separator();
                    });

                egui::CollapsingHeader::new("Render Order")
                    .default_open(true)
                    .show(ui, |ui| {
                        let draw_node =
                            |ui: &mut egui::Ui,
                             node_id: usize,
                             node: &mut gametoy::config_file::Node| {
                                let area_name = &format!(
                                    "{} ({})",
                                    graph::get_node_name(&node),
                                    graph::get_node_type_name(&node)
                                );

                                let available_space = ui.available_size();
                                if ui
                                    .add_sized(available_space, egui::Button::new(area_name))
                                    .clicked()
                                {
                                    self.selected_node_id = Some(node_id);
                                };
                            };
                        helpers::list_edit(ui, &mut new_proj.graph.nodes, draw_node);

                        ui.separator();
                        ui.label("Add Node");

                        super::graph::add_node_widget(&mut new_proj.graph.nodes, ui);
                        ui.separator();
                    });
                egui::CollapsingHeader::new("Node Properties")
                    .default_open(true)
                    .show(ui, |ui| {
                        match self.selected_node_id {
                            Some(id) => match new_proj.graph.nodes.get_mut(id) {
                                Some(node) => {
                                    graph::draw_node_properties(node, ui);
                                }
                                None => {
                                    self.selected_node_id = None;
                                }
                            },
                            None => {
                                ui.label("Select a Node");
                            }
                        };
                        ui.separator();
                    })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("egui template");
            ui.hyperlink("https://github.com/emilk/egui_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/egui_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });
        if new_proj != self.project_data {
            self.project_data = new_proj;
            self.dirty = true;
        }
    }
}

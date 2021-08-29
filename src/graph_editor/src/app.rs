use eframe::{egui, epi};
use gametoy::gamedata::CONFIG_FILE_NAME;
use rfd::FileDialog;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;


use super::nodes;
use super::helpers;
use super::metadata;
use super::state;

use super::state::StateOperation;

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
            dirty: false
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
                    self.reactor.queue_operation(state::StateOperation::SetProjectPath(Some(filepath)));
                    self.reactor.queue_operation(state::StateOperation::LoadFromConfigFile(conf));
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
        if let Err(err) = save_data_file(&self.state.project_data, filepath) {
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
        if let Some(output_file) = dialog.save_file(){
            self.save_file(&output_file);
            self.reactor.queue_operation(state::StateOperation::SetProjectPath(Some(output_file)));
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
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {

        //let mut new_proj = self.state.project_data.clone();

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
                        metadata::draw_metadata(ui, &self.state, &mut self.reactor);
                        ui.separator();
                    });

                egui::CollapsingHeader::new("Render Order")
                    .default_open(true)
                    .show(ui, |ui| {

                        let reactor = &mut self.reactor;
                        let nodes = &self.state.project_data.graph.nodes;

                        let draw_node =
                            |ui: &mut egui::Ui,
                             node_id: usize,
                             node: &gametoy::config_file::Node| {
                                let area_name = &format!(
                                    "{} ({})",
                                    nodes::get_node_name(&node),
                                    nodes::get_node_type_name(&node)
                                );

                                let available_space = ui.available_size();
                                if ui
                                    .add_sized(available_space, egui::Button::new(area_name))
                                    .clicked()
                                {
                                    reactor.queue_operation(StateOperation::SelectNode(Some(node_id)));
                                };
                            };
 
                        match helpers::list_edit(ui, nodes, draw_node, "render_order_grid") {
                            helpers::ListEditResponse::None => {},
                            helpers::ListEditResponse::Remove(node_id) => self.reactor.queue_operation(StateOperation::DeleteNode(node_id)),
                            helpers::ListEditResponse::Swap(node_id_1, node_id_2) => self.reactor.queue_operation(StateOperation::SwapNodes(node_id_1, node_id_2))
                        };

                        ui.separator();
                        ui.label("Add Node");

                        super::nodes::add_node_widget(ui, &self.state, &mut self.reactor);
                        ui.separator();
                    });
                    
                    egui::CollapsingHeader::new("Node Properties")
                    .default_open(true)
                    .show(ui, |ui| {
                        match self.state.selected_node_id {
                            Some(id) => match self.state.project_data.graph.nodes.get(id) {
                                Some(node) => {
                                    nodes::draw_node_properties(ui, &mut self.reactor, node, id);
                                }
                                None => {
                                    self.reactor.queue_operation(StateOperation::SelectNode(None));
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
            super::graph::draw_rendergraph_editor(ui, &mut self.reactor, &mut self.state.node_context, &self.state.project_data);
        });
        

        let old_project_state = self.state.project_data.clone();
        self.reactor.react(&mut self.state);
        if old_project_state != self.state.project_data {
            self.dirty = true
        }
        
    }
}



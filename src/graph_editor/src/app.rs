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
                                    nodes::get_node_name(&node),
                                    nodes::get_node_type_name(&node)
                                );

                                let available_space = ui.available_size();
                                if ui
                                    .add_sized(available_space, egui::Button::new(area_name))
                                    .clicked()
                                {
                                    self.selected_node_id = Some(node_id);
                                };
                            };
                        helpers::list_edit(ui, &mut new_proj.graph.nodes, draw_node, "render_order_grid");

                        ui.separator();
                        ui.label("Add Node");

                        super::nodes::add_node_widget(&mut new_proj.graph.nodes, ui);
                        ui.separator();
                    });
                egui::CollapsingHeader::new("Node Properties")
                    .default_open(true)
                    .show(ui, |ui| {
                        match self.selected_node_id {
                            Some(id) => match new_proj.graph.nodes.get_mut(id) {
                                Some(node) => {
                                    nodes::draw_node_properties(node, ui);
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

            for (node_id, node) in new_proj.graph.nodes.iter().enumerate() {
                let window_id = format!("{} ({})", nodes::get_node_name(&node), node_id);
                egui::Window::new(&window_id).collapsible(false).show(ctx, |ui| {

                    egui::Grid::new(format!("{}grid",window_id))
                        .num_columns(2).min_col_width(100.0).show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                                ui.add(egui::Label::new("Inputs").weak());
                                for input_slot in get_input_slots(&node) {
                                    ui.horizontal(|ui| {
                                        ui.add(input_connector());
                                        ui.add(egui::Label::new(input_slot));
                                    });
                                    
                                }

                            });
                            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                                    ui.add(egui::Label::new("Outputs").weak());
                                    for output_slot in get_output_slots(&node) {
                                        ui.horizontal(|ui| {
                                            ui.add(input_connector());
                                            ui.add(egui::Label::new(output_slot));
                                        });
                                    }
                            });
                        });

                 });
            }
            
        });
        if new_proj != self.project_data {
            self.project_data = new_proj;
            self.dirty = true;
        }
    }
}

fn get_input_slots(node: &gametoy::config_file::Node) -> Vec<String>{
    match node {
        gametoy::config_file::Node::Image(_image_data) => vec![],
        gametoy::config_file::Node::Keyboard(_keyboard_data) => vec![],
        gametoy::config_file::Node::Output(_output_data) => vec![gametoy::nodes::Output::INPUT_BUFFER_NAME.to_string()],
        gametoy::config_file::Node::RenderPass(renderpass_data) => renderpass_data.input_texture_slots.iter().map(|x| x.name.clone()).collect(),
    }

}

fn get_output_slots(node: &gametoy::config_file::Node) -> Vec<String>{
    match node {
        gametoy::config_file::Node::Image(_image_data) => vec![gametoy::nodes::Image::OUTPUT_BUFFER_NAME.to_string()],
        gametoy::config_file::Node::Keyboard(_keyboard_data) => vec![gametoy::nodes::Keyboard::OUTPUT_BUFFER_NAME.to_string()],
        gametoy::config_file::Node::Output(_output_data) => vec![],
        gametoy::config_file::Node::RenderPass(renderpass_data) => renderpass_data.output_texture_slots.iter().map(|x| x.name.clone()).collect(),
    }

}

fn input_connector_internals(ui: &mut egui::Ui) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, true, ""));

    let visuals = ui.style().interact_selectable(&response, true);
    let rect = rect.expand(visuals.expansion);
    let radius = 0.5 * rect.height();
    ui.painter().circle(rect.center(), 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    response
}

pub fn input_connector() -> impl egui::Widget {
    move |ui: &mut egui::Ui| input_connector_internals(ui)
}
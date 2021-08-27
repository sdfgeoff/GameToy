use eframe::{egui, epi};
use gametoy::gamedata::CONFIG_FILE_NAME;
use rfd::FileDialog;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use egui_nodes::{NodeConstructor, LinkArgs};

use super::nodes;
use super::helpers;
use super::metadata;

pub struct GametoyGraphEditor {
    project_file: Option<PathBuf>,
    project_data: gametoy::config_file::ConfigFile,
    dirty: bool,
    selected_node_id: Option<usize>,
    node_context: egui_nodes::Context,
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
            links: vec![
                gametoy::config_file::Link {
                    start_node: "Keyboard".to_string(),
                    start_output_slot: "tex".to_string(),
                    end_node: "Render Pass 1".to_string(),
                    end_input_slot: "KeyboardInput".to_string(),
                },
            ],
        },
    }
}

impl Default for GametoyGraphEditor {
    fn default() -> Self {
        let mut context = egui_nodes::Context::default();
        context.io.link_detatch_with_modifier_click = egui_nodes::Modifiers::Alt;
        Self {
            project_file: None,
            project_data: create_default_project(),
            dirty: false,
            selected_node_id: None,
            node_context: context
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
            } else {
                self.dirty = false;
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
            let graph_nodes = &mut new_proj.graph.nodes;
            let graph_links = &mut new_proj.graph.links;

            let nodes: Vec<NodeConstructor> = graph_nodes.iter().enumerate().map(|(node_id, node)| {
                let title = format!("({}) {}", node_id, nodes::get_node_name(&node));

                let mut node_constructor = NodeConstructor::new(node_id, Default::default())
                    .with_title(move |ui| ui.label(title));

                
                let input_slots = get_input_slots(&node);
                for (slot_id, input_name) in input_slots.iter().enumerate() {
                   let slot_name = input_name.to_string();
                   let pin_id = pairing_function(node_id, slot_id);
                   node_constructor = node_constructor.with_input_attribute(pin_id, Default::default(), move |ui| ui.label(slot_name));
                }
                for (slot_id, output_name) in get_output_slots(&node).iter().enumerate() {
                    let slot_name = output_name.clone();
                    let pin_id = pairing_function(node_id, slot_id + input_slots.len());
                    node_constructor = node_constructor.with_output_attribute(pin_id, Default::default(), move |ui| ui.label(slot_name));
                }
                node_constructor
            }).collect();

            // With a change in file format a lot of this complexity could be removed
            // Here we convert from names of nodes + names of slots into ID's
            // (and in a short distance we convert back)
            use std::collections::HashMap;

            let links = &mut vec![];
            let mut node_name_to_id: HashMap<String, usize> = HashMap::new();
            for (node_id, node) in graph_nodes.iter().enumerate() {
                let node_name = nodes::get_node_name(node);
                node_name_to_id.insert(node_name.to_string(), node_id);
            }

            for link in graph_links {
                if let Some(start_node_id) = node_name_to_id.get(&link.start_node) {
                    if let Some(end_node_id) = node_name_to_id.get(&link.end_node) {
                        if let Some(start_slot_id) = get_output_slots(&graph_nodes[*start_node_id]).iter().position(|x| {*x == link.start_output_slot}) {
                            if let Some(end_slot_id) = get_input_slots(&graph_nodes[*end_node_id]).iter().position(|x| {*x == link.end_input_slot}) {

                                let input_slots = get_input_slots(&graph_nodes[*start_node_id]);
                                let start_pin = pairing_function(*start_node_id, start_slot_id + input_slots.len());
                                let end_pin = pairing_function(*end_node_id, end_slot_id);
                                links.push((
                                    start_pin,
                                    end_pin,
                                ));
                            }
                        }
                    }
                }
                
            }
            

            // add them to the ui
            self.node_context.show(
                nodes,
                links.iter().enumerate().map(|(i, (start, end))| (i, *start, *end, LinkArgs::default())),
                ui
            );
            
            // remove destroyed links
            if let Some(idx) = self.node_context.link_destroyed() {
                println!("del: {}", idx);
                links.remove(idx);
            }
        
            // add created links
            if let Some((start, end, _)) = self.node_context.link_created() {
                let (start_node_id, start_slot_id_and_len) = unpairing_function(start);
                let (end_node_id, end_slot_id) = unpairing_function(end);
                let start_node = &graph_nodes[start_node_id];
                let end_node = &graph_nodes[end_node_id];
                let start_slot_id = start_slot_id_and_len - get_input_slots(&start_node).len();

                let start_node_name = nodes::get_node_name(&start_node).to_string();
                let end_node_name = nodes::get_node_name(&end_node).to_string();
                let start_output_slot_name = get_output_slots(&start_node)[start_slot_id].clone();
                let end_input_slot_name = get_input_slots(&end_node)[end_slot_id].clone();

                let link_to_create = gametoy::config_file::Link {
                    start_node: start_node_name,
                    start_output_slot: start_output_slot_name,
                    end_node: end_node_name,
                    end_input_slot: end_input_slot_name,
                };
                // Remove old links that link to the same place:
                new_proj.graph.links = new_proj.graph.links.iter().filter(|existing_link| {
                    if existing_link.end_node == link_to_create.end_node && existing_link.end_input_slot == link_to_create.end_input_slot {
                        false
                    } else {
                        true
                    }
                }).map(|l| {l.clone()}).collect();
                new_proj.graph.links.push(link_to_create);
            }

            // Ensure links are to existing slots/nodes:
            new_proj.graph.links = new_proj.graph.links.iter().filter(|existing_link| {
                if let Some(start_node_id) = node_name_to_id.get(&existing_link.start_node) {
                    if let Some(end_node_id) = node_name_to_id.get(&existing_link.end_node) {
                        if get_output_slots(&graph_nodes[*start_node_id]).contains(&existing_link.start_output_slot) {
                            if get_input_slots(&graph_nodes[*end_node_id]).contains(&existing_link.end_input_slot) {
                                return true
                            }
                        }
                    }
                }
                return false
            }).map(|l| {l.clone()}).collect();

            if let Some(selected_node) = self.node_context.get_selected_nodes().pop() {
                self.selected_node_id = Some(selected_node);
            }
        });



        if new_proj != self.project_data {
            self.project_data = new_proj;
            self.dirty = true;
        }
    }
}

/// An Elegant Pairing Function by Matthew Szudzik @ Wolfram Research, Inc.
fn pairing_function(x: usize, y: usize) -> usize {
    if x >= y {
        x * x + x + y
    } else {
        y * y + x
    }
}
fn unpairing_function(z: usize) -> (usize, usize) {
    let sqrtz = (f64::sqrt(z as f64)).floor() as usize;
    let sqz = sqrtz * sqrtz;
    if (z - sqz) >= sqrtz {
        (sqrtz, z - sqz - sqrtz)
    } else {
        (z - sqz, sqrtz)
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
use gametoy::config_file::{ConfigFile, Link, MetaData, Node};
use std::path::PathBuf;
use std::collections::HashMap;

pub mod templates;

#[derive(Debug)]
pub enum UiLayoutMode {
    GraphEditor,
    TextEditor(String),
}

impl Default for UiLayoutMode {
    fn default() -> Self {
        Self::GraphEditor
    }
}

pub struct GamePlayState {
    // pub playing: bool,
    pub render_size: [u32;2],
}

#[derive(Default)]
pub struct UiState {
    pub selected_node_id: Option<usize>,
    pub node_context: egui_nodes::Context,
    pub ui_layout_mode: UiLayoutMode,
}

#[derive(Clone, PartialEq)]
pub struct ProjectData {
    pub config_file: ConfigFile,
    pub files: HashMap<String, Vec<u8>>
}

pub struct EditorState {
    /// Anything tracked by the undo system, saved to disk with the project etc.
    /// must be in here
    pub project_data: ProjectData,

    // --------------- Temporary State -------------------
    /// File path at which this project is stored
    pub project_file: Option<PathBuf>,
    pub game_play_state: GamePlayState,
    pub ui_state: UiState,
}

pub enum StateOperation {
    SetProjectPath(Option<PathBuf>),
    LoadFromConfigFile(ConfigFile),
    SelectNode(Option<usize>),
    SetMetadata(MetaData),
    CreateNode(Node),
    DeleteNode(usize),
    SwapNodes(usize, usize),
    UpdateNode(usize, Node),
    CreateLink(Link),
    DeleteLink(usize),
    RemoveInvalidLinks,
    SetGameRenderSize([u32;2]),
    SetUiLayoutMode(UiLayoutMode),
    WriteToFile(String, Vec<u8>)
}

pub struct Reactor {
    operation_queue: Vec<StateOperation>,
}

impl Reactor {
    pub fn new() -> Self {
        Self {
            operation_queue: vec![],
        }
    }

    pub fn queue_operation(&mut self, operation: StateOperation) {
        self.operation_queue.push(operation);
    }

    pub fn react(&mut self, state: &mut EditorState) {
        while let Some(operation) = self.operation_queue.pop() {
            perform_operation(state, operation);
        }
    }
}

pub fn perform_operation(state: &mut EditorState, operation: StateOperation) {
    match operation {
        StateOperation::SetProjectPath(new_path) => {
            state.project_file = new_path;
        }
        StateOperation::LoadFromConfigFile(conf) => {
            state.project_data.config_file = conf;
            // TODO: load associated resources
        }
        StateOperation::SelectNode(node_id) => {
            state.ui_state.selected_node_id = node_id;
            if let Some(id) = node_id {
                if !state.ui_state.node_context.selected_node_indices.contains(&id) {
                    state.ui_state.node_context.selected_node_indices = vec![id];
                }
            }
        }
        StateOperation::SetMetadata(metadata) => {
            state.project_data.config_file.metadata = metadata;
        }
        StateOperation::CreateNode(node) => {
            state.project_data.config_file.graph.nodes.push(node);
        }
        StateOperation::DeleteNode(node_id) => {
            state.project_data.config_file.graph.nodes.remove(node_id);
        }
        StateOperation::SwapNodes(node_id_1, node_id_2) => {
            let num_nodes = state.project_data.config_file.graph.nodes.len();
            if node_id_1 < num_nodes && node_id_2 < num_nodes {
                state.project_data.config_file.graph.nodes.swap(node_id_1, node_id_2);
            } else {
                println!("Warn: unable to swap");
            }
        }
        StateOperation::UpdateNode(node_id, new_node_data) => {
            // TODO: Bounds check and check for the name changing
            {
                let old_node_data = &state.project_data.config_file.graph.nodes[node_id];
                let old_node_name = crate::nodes::get_node_name(&old_node_data);
                let new_node_name = crate::nodes::get_node_name(&new_node_data);

                for link in state.project_data.config_file.graph.links.iter_mut() {
                    if link.start_node == old_node_name {
                        link.start_node = new_node_name.to_string();
                    }
                    if link.end_node == old_node_name {
                        link.end_node = new_node_name.to_string();
                    }
                    // TODO: Check for link names changing as well
                }
            }
            state.project_data.config_file.graph.nodes[node_id] = new_node_data
        }
        StateOperation::CreateLink(link) => {
            state.project_data.config_file.graph.links.push(link);
        }
        StateOperation::DeleteLink(link_id) => {
            state.project_data.config_file.graph.links.remove(link_id);
        }
        StateOperation::RemoveInvalidLinks => {
            // Ensure links are to existing slots/nodes:
            let mut node_name_to_id: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            for (node_id, node) in state.project_data.config_file.graph.nodes.iter().enumerate() {
                let node_name = crate::nodes::get_node_name(node);
                node_name_to_id.insert(node_name.to_string(), node_id);
            }

            let graph_nodes = &state.project_data.config_file.graph.nodes;

            state.project_data.config_file.graph.links.retain(|existing_link|{
                if let Some(start_node_id) = node_name_to_id.get(&existing_link.start_node) {
                    if let Some(end_node_id) = node_name_to_id.get(&existing_link.end_node) {
                        let start_node = &graph_nodes[*start_node_id];
                        if crate::nodes::get_output_slots(start_node).contains(&existing_link.start_output_slot) {
                            let end_node = &graph_nodes[*end_node_id];
                            if crate::nodes::get_input_slots(end_node).contains(&existing_link.end_input_slot) {
                                return true;
                            }
                        }
                    }
                }
                false
            })
        }
        StateOperation::SetGameRenderSize(size) => {
            state.game_play_state.render_size = size;
        }
        StateOperation::SetUiLayoutMode(mode) => {
            state.ui_state.ui_layout_mode = mode;
        }

        StateOperation::WriteToFile(filename, buffer) => {
            state.project_data.files.insert(filename, buffer);
        }
    }
}

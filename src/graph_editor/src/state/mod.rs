use gametoy::config_file::{ConfigFile, Link, MetaData, Node};
use std::path::PathBuf;

pub mod templates;

pub struct EditorState {
    /// File path at which this project is stored
    pub project_file: Option<PathBuf>,

    /// Anything tracked by the undo system, saved to disk with the project etc.
    /// must be in here
    pub project_data: ConfigFile,

    // --------------- UI State -------------------
    pub selected_node_id: Option<usize>,
    pub node_context: egui_nodes::Context,
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
            state.project_data = conf;
        }
        StateOperation::SelectNode(node_id) => {
            state.selected_node_id = node_id;
            if let Some(id) = node_id {
                if !state.node_context.selected_node_indices.contains(&id) {
                    state.node_context.selected_node_indices = vec![id];
                }
            }
        }
        StateOperation::SetMetadata(metadata) => {
            state.project_data.metadata = metadata;
        }
        StateOperation::CreateNode(node) => {
            state.project_data.graph.nodes.push(node);
        }
        StateOperation::DeleteNode(node_id) => {
            state.project_data.graph.nodes.remove(node_id);
        }
        StateOperation::SwapNodes(node_id_1, node_id_2) => {
            let num_nodes = state.project_data.graph.nodes.len();
            if node_id_1 < num_nodes && node_id_2 < num_nodes {
                state.project_data.graph.nodes.swap(node_id_1, node_id_2);
            } else {
                println!("Warn: unable to swap");
            }
        }
        StateOperation::UpdateNode(node_id, new_node_data) => {
            // TODO: Bounds check and check for the name changing
            let old_node_name = crate::nodes::get_node_name(&state.project_data.graph.nodes[node_id]);
            let new_node_name = crate::nodes::get_node_name(&new_node_data);
            if old_node_name != new_node_name {
                for link in state.project_data.graph.links.iter_mut() {
                    if link.start_node == old_node_name {
                        link.start_node = new_node_name.to_string();
                    }
                    if link.end_node == old_node_name {
                        link.end_node = new_node_name.to_string();
                    }
                }
            }
            // TODO: Check for link names changing as well
            state.project_data.graph.nodes[node_id] = new_node_data
        }
        StateOperation::CreateLink(link) => {
            state.project_data.graph.links.push(link);
        }
        StateOperation::DeleteLink(link_id) => {
            state.project_data.graph.links.remove(link_id);
        }
    }
}

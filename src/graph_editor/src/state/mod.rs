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

    pub gametoy_instance: Option<Result<gametoy::GameToy, gametoy::GameToyError>>,
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
    WriteToFile(String, Vec<u8>),
    CompileGametoy,
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

    pub fn react(&mut self, state: &mut EditorState, gl: &glow::Context) {
        while let Some(operation) = self.operation_queue.pop() {
            perform_operation(state, operation, gl);
        }
    }
}

pub fn perform_operation(state: &mut EditorState, operation: StateOperation, gl: &glow::Context) {
    match operation {
        StateOperation::SetProjectPath(new_path) => {
            state.project_file = new_path;
        }
        StateOperation::LoadFromConfigFile(conf) => {
            state.project_data.config_file = conf;
            state.gametoy_instance = None;
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
                let node_pos_1 = state.ui_state.node_context.get_node_pos_screen_space(node_id_1);
                let node_pos_2 = state.ui_state.node_context.get_node_pos_screen_space(node_id_2);
                if let Some(pos) = node_pos_1 {
                    state.ui_state.node_context.set_node_pos_screen_space(node_id_2, pos);
                }
                if let Some(pos) = node_pos_2 {
                    state.ui_state.node_context.set_node_pos_screen_space(node_id_1, pos);
                }
            } else {
                println!("Warn: unable to swap");
            }
        }
        StateOperation::UpdateNode(node_id, new_node_data) => {
            // TODO: Bounds check and check for the name changing
            {
                let old_node_data = &state.project_data.config_file.graph.nodes[node_id];
                let new_node_name = crate::nodes::get_node_name(&new_node_data);

                {
                    // Changing Node Names
                    let old_node_name = crate::nodes::get_node_name(&old_node_data);
                    if old_node_name != new_node_name {
                        for link in state.project_data.config_file.graph.links.iter_mut() {
                            if link.start_node == old_node_name {
                                link.start_node = new_node_name.to_string();
                            }
                            if link.end_node == old_node_name {
                                link.end_node = new_node_name.to_string();
                            }
                        }
                    }
                }


                {
                    // Changing Output Link Names
                    let old_link_names = crate::nodes::get_output_slots(&old_node_data);
                    let new_link_names = crate::nodes::get_output_slots(&new_node_data);
                    if old_link_names.len() == new_link_names.len() {
                        for (old, new) in old_link_names.iter().zip(new_link_names.iter()) {
                            if old != new && !new_link_names.contains(old) {
                                for link in state.project_data.config_file.graph.links.iter_mut() {
                                    if link.start_node == new_node_name && &link.start_output_slot == old {
                                        link.start_output_slot = new.to_string();
                                    }
                                }
                            }
                        }
                    }
                }

                {
                    // Changing Input Link Names
                    let old_link_names = crate::nodes::get_input_slots(&old_node_data);
                    let new_link_names = crate::nodes::get_input_slots(&new_node_data);
                    if old_link_names.len() == new_link_names.len() {
                        for (old, new) in old_link_names.iter().zip(new_link_names.iter()) {
                            if old != new && !new_link_names.contains(old) {
                                for link in state.project_data.config_file.graph.links.iter_mut() {
                                    if link.end_node == new_node_name && &link.end_input_slot == old {
                                        link.end_input_slot = new.to_string();
                                    }
                                }
                            }
                        }
                    }
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

        StateOperation::CompileGametoy => {
            // First we create a TAR of all the assets
            if state.gametoy_instance.is_some(){
                todo!("Implement destruction of gametoy");
            }
            let instance = create_gametoy_instance(&state.project_data, gl);
            unsafe {
                use glow::HasContext;
                gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            }
            state.gametoy_instance = Some(instance);
        }
    }
}


fn create_gametoy_instance(project_data: &ProjectData, gl: &glow::Context) -> Result<gametoy::GameToy, gametoy::GameToyError> {

    let mut tarfile = gametoy::tar::Builder::new(Vec::new());

    for (filename, filedata) in project_data.files.iter() {
        let mut header = gametoy::tar::Header::new_gnu();
        header.set_size(filedata.len() as u64);
        header.set_cksum();

        tarfile.append_data(&mut header, filename, filedata.as_slice()).expect("Failed to pack into tar");
    }

    {
        let config_file_str = serde_json::to_string(&project_data.config_file).expect("Failed to serialize config file");
        let config_file_bytes = config_file_str.as_bytes();
        let mut header = gametoy::tar::Header::new_gnu();
        header.set_size(config_file_bytes.len() as u64);
        header.set_cksum();

        tarfile.append_data(&mut header, "data.json", config_file_bytes).expect("Failed to pack into tar");

    }
    

    let tardata = tarfile.into_inner().expect("Failed to create archive");
    let tarchive = gametoy::tar::Archive::new(tardata.as_slice());


    gametoy::GameToy::new(gl, tarchive, false)
    //Err(gametoy::GameToyError::DuplicateNodeName("DuplicateNodeName".to_string()))
}

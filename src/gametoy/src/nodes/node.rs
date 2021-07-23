use glow::Texture;

#[derive(Debug)]
pub enum NodeError {
    /// This node does not contain an input texture with the supplied name.
    /// String is the name of the texture requested
    NoSuchInputTexture(String),

    /// This node does not contain an output texture with the supplied name
    /// String is the name of the texture requested
    NoSuchOutputTexture(String),

    /// The GPU failed to allocate a framebuffer
    CreateFramebufferFailed(String),

    /// The GPU failed to allocate a texture
    CreateTextureFailed(String),

    /// The text files that should contain the shader source code
    /// do not exist in the supplied GameData
    MissingResource(String),

    /// This renderpass has two input slots with the same name
    DuplicateInputSlotName(String),

    /// This renderpass has two output slots with the same name
    DuplicateOutputSlotName(String),

    /// There is no shader defined for this renderpass!
    NoShader,

    /// Shader failed to compile/link etc.
    ShaderError(crate::shader::ShaderError),
}

pub trait Node {
    /// Returns the name of this node
    fn get_name(&self) -> &String;

    /// Sets up this node for rendering. A following call with a quad should render this
    /// node
    fn bind(&mut self, gl: &glow::Context, quad: &crate::quad::Quad, game_state: &crate::GameState);

    /// Run when the screen resolution has changed. This indicates that the node may
    /// need to update it's resolution as well.
    fn update_resolution(&mut self, gl: &glow::Context, screen_resolution: &[i32; 2]);

    /// Returns the texture that this node outputs with the provided name. If there is no
    /// such texture, it returns the NodeError::NoSuchOutputTexture error.
    fn get_output_texture(&self, name: &String) -> Result<Texture, NodeError>;

    /// Sets the input texture. If there is no texture with the name, it returns the NodeError::NoSuchInputTexture error
    fn set_input_texture(&mut self, name: &String, texture: Texture) -> Result<(), NodeError>;

    /// If a node has it's own output connected to it's own input (aka self-referential), then some
    /// nodes will need to take special action (eg double buffering). This function is called after
    /// the node is created to allow errors to be thrown or the node to configure itself.
    fn set_up_self_reference(
        &mut self,
        _gl: &glow::Context,
        _input_slot_name: &String,
        _output_slot_name: &String,
    ) -> Result<(), NodeError> {
        Ok(())
    }
}

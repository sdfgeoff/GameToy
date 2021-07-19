use glow::Texture;

#[derive(Debug)]
pub enum NodeError {
    NoSuchInputTexture(String),
    NoSuchOutputTexture(String),
}

pub trait Node {
    /// Returns the name of this node
    fn get_name(&self) -> &String;

    /// Sets up this node for rendering. A following call with a quad should render this
    /// node
    fn bind(&mut self, gl: &glow::Context);

    /// Run when the screen resolution has changed. This indicates that the node may
    /// need to update it's resolution as well.
    fn update_resolution(&mut self, gl: &glow::Context, screen_resolution: &[i32; 2]);

    /// Returns the texture that this node outputs with the provided name. If there is no
    /// such texture, it returns the NodeError::NoSuchOutputTexture error.
    fn get_output_texture(&self, name: &String) -> Result<Texture, NodeError>;

    /// Sets the input texture. If there is no texture with the name, it returns the NodeError::NoSuchInputTexture error
    fn set_input_texture(&mut self, name: &String, texture: Texture) -> Result<(), NodeError>;
}

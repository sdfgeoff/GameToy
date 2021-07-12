pub trait Node {
    //fn get_output_texture(&self, name: String) -> Option<Texture>;

    fn get_name(&self) -> &String;
    //fn handle_resize(&self, viewport_size_x: u32, viewport_size_y: u32);

    fn bind(&mut self, gl: &glow::Context);

    fn update_resolution(&mut self, gl: &glow::Context, screen_resolution: &[i32; 2]);
}
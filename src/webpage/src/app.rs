use js_sys::Date;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, KeyboardEvent};

use gametoy;
use gametoy::glow;
use gametoy::tar::Archive;

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    canvas: HtmlCanvasElement,
    toy: gametoy::GameToy,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement, tar_data: Vec<u8>) -> Self {
        log("[OK] Got App");
        let (gl, shader_version) = {
            let webgl2_context = canvas
                .get_context("webgl2")
                .expect("Failed to get context 1")
                .expect("Failed to get context 2")
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .expect("Failed to get context 3");
            log("[OK] Got Context");

            // Grab various extensions....
            let _float_texture_ext = webgl2_context.get_extension("OES_texture_float");
            let _float_texture_ext = webgl2_context.get_extension("EXT_color_buffer_float");

            let gl = glow::Context::from_webgl2_context(webgl2_context);
            (gl, "#version 300 es")
        };
        log("[OK] Got GL");

        let tar = Archive::new(tar_data.as_slice());
        log("[OK] Got Tar");

        let toy = gametoy::GameToy::new(gl, tar).expect("Failed to create toy");

        Self { canvas, toy }
    }

    fn check_resize(&mut self) {
        let client_width = self.canvas.client_width();
        let client_height = self.canvas.client_height();
        let canvas_width = self.canvas.width() as i32;
        let canvas_height = self.canvas.height() as i32;

        if client_width != canvas_width || client_height != canvas_height {
            let client_width = client_width as u32;
            let client_height = client_height as u32;

            self.canvas.set_width(client_width);
            self.canvas.set_height(client_height);

            self.toy.resize(client_width, client_height);

            log(&format!(
                "[OK] Resized to {}:{}",
                client_width, client_height
            ));
        }
    }

    pub fn animation_frame(&mut self) {
        self.check_resize();

        let time = Date::new_0().get_time() / 1000.0;
        self.toy.render(time).expect("Failed to render toy");
    }

    pub fn keydown_event(&mut self, event: KeyboardEvent) {
        self.toy.set_key_state(event.which(), true);
    }
    pub fn keyup_event(&mut self, event: KeyboardEvent) {
        self.toy.set_key_state(event.which(), false);
    }
}

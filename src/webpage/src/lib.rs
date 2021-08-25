use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{ArrayBuffer, Function, Uint8Array};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    window, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent, Request, RequestInit, RequestMode,
    Response,
};

mod app;

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// This struct will be accessible from JS as a JS object that can be
// created using `new Core()`
#[wasm_bindgen]
pub struct Core {
    app: Rc<RefCell<app::App>>,
    canvas: HtmlCanvasElement,
}

pub async fn load_tar() -> Vec<u8> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = "../datapack.tar";

    log(&format!("[OK] Downloading datapack from {}", &url));

    let request = Request::new_with_str_and_init(&url, &opts).expect("Failed to create request");

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")
        .expect("Failed to set headers");

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .expect("Failed to get request");

    log(&format!("[OK] Download Completed. Loading to WASM"));

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let array_promise = resp.array_buffer().unwrap();
    let array = wasm_bindgen_futures::JsFuture::from(array_promise)
        .await
        .expect("Failed to get array buffer");
    assert!(array.is_instance_of::<ArrayBuffer>());
    let uint8_view = Uint8Array::new_with_byte_offset(&array, 0);

    uint8_view.to_vec()
}

#[wasm_bindgen]
pub async fn load_core(canvas: HtmlCanvasElement) -> Core {
    log(&format!("WASM Started for canvas {}", canvas.id()));

    let tar_data = load_tar().await;

    log("[OK] DataPack in RAM, Starting Core");
    let mut core = Core::new(canvas, tar_data);
    core.start();

    core
}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement, tar_data: Vec<u8>) -> Self {
        //console_error_panic_hook::set_once(); // Adds 4kb
        canvas.set_class_name("game loaded");

        let app = Rc::new(RefCell::new(app::App::new(canvas.clone(), tar_data)));

        Self { app, canvas }
    }

    pub fn start(&mut self) {
        let window = window().unwrap();
        {
            // Animation Frame
            let callback = Rc::new(RefCell::new(None));

            let anim_app = self.app.clone();
            let anim_window = window.clone();
            let anim_callback = callback.clone();

            *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                anim_app.borrow_mut().animation_frame();
                // Schedule ourself for another requestAnimationFrame callback.
                anim_window
                    .request_animation_frame(make_callback(
                        anim_callback.borrow().as_ref().unwrap(),
                    ))
                    .unwrap();
            }) as Box<dyn FnMut()>));
            window
                .request_animation_frame(make_callback(callback.borrow().as_ref().unwrap()))
                .unwrap();
        }

        {
            // keyboard events
            self.canvas.set_tab_index(1); // Canvas elements ignore key events unless they have a tab index
            let anim_app1 = self.app.clone();
            let anim_app2 = self.app.clone();

            let keydown_callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                let e: Event = event.clone().dyn_into().unwrap();
                e.stop_propagation();
                e.prevent_default();

                anim_app1.borrow_mut().keydown_event(event);
            }) as Box<dyn FnMut(_)>);

            let keyup_callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                let e: Event = event.clone().dyn_into().unwrap();
                e.stop_propagation();
                e.prevent_default();

                anim_app2.borrow_mut().keyup_event(event);
            }) as Box<dyn FnMut(_)>);

            self.canvas
                .add_event_listener_with_callback(
                    "keydown",
                    keydown_callback.as_ref().unchecked_ref(),
                )
                .unwrap();

            self.canvas
                .add_event_listener_with_callback("keyup", keyup_callback.as_ref().unchecked_ref())
                .unwrap();

            keydown_callback.forget();
            keyup_callback.forget();
        }
    }
}

fn make_callback(closure: &Closure<dyn FnMut()>) -> &Function {
    closure.as_ref().unchecked_ref()
}

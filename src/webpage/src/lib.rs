use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Function;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{window, Event, HtmlCanvasElement, KeyEvent, MouseEvent};

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

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        log(&format!("WASM Started for canvas {}", canvas.id()));
        console_error_panic_hook::set_once();
        canvas.set_class_name("game loaded");

        let app = Rc::new(RefCell::new(app::App::new(canvas.clone())));
        
        Self {
            app,
            canvas
        }
    }

    #[wasm_bindgen]
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
    }
}

fn make_callback(closure: &Closure<dyn FnMut()>) -> &Function {
    closure.as_ref().unchecked_ref()
}

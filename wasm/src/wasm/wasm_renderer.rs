use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use teki_common::traits::Renderer;
use teki_common::utils::collision::VRect;
use vector2d::Vector2D;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlImageElement};

#[wasm_bindgen]
pub struct WasmRenderer {
    canvas: HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    images: Rc<RefCell<HashMap<String, HtmlImageElement>>>,
}

#[wasm_bindgen]
impl WasmRenderer {
    pub fn new(canvas_id: &str) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: HtmlCanvasElement =
            canvas.dyn_into::<HtmlCanvasElement>().map_err(|_| ()).unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self { canvas, context, images: Rc::new(RefCell::new(HashMap::new())) }
    }
}

impl Renderer for WasmRenderer {
    fn load_sprite(&mut self, path: &str, vrect: VRect) {
        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

        let basename = String::from(Path::new(path).file_stem().unwrap().to_str().unwrap());
        {
            let basename = basename.clone();
            let images = self.images.clone();
            let image_dup = image.clone();
            let closure = Closure::once_into_js(move |_event: JsValue| {
                image_dup.borrow_mut().set_onerror(None);
                image_dup.borrow_mut().set_onload(None);

                let image = Rc::try_unwrap(image_dup).unwrap().into_inner();
                images.borrow_mut().insert(basename, image);
            });
            let cb = closure.as_ref().unchecked_ref();
            image.borrow_mut().set_onload(Some(cb));
        }
        image.borrow_mut().set_src(&path);
    }

    fn set_draw_gradient(&mut self) {}

    fn clear(&mut self) {
        self.context.fill_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64)
    }

    fn draw_str(&mut self, path: &str, x: i32, y: i32, text: &str, r: u8, g: u8, b: u8) {
        let image = self.images.borrow();
        if let Some(image) = image.get(path) {
            let mut x = x as f64;
            let y = y as f64;
            let w = 8.0;
            let h = 8.0;
            self.context.set_fill_style(&JsValue::from("rgb(255,0,0)"));
            for c in text.chars() {
                let u: i32 = ((c as i32) - (' ' as i32)) % 16 * 8;
                let v: i32 = ((c as i32) - (' ' as i32)) / 16 * 8;
                self.context
                    .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        &image, u as f64, v as f64, w, h, x, y, w, h,
                    )
                    .expect("draw_image_with... failed");
                x += w;
            }
        }
    }
    fn draw_sprite(&mut self, sprite_name: &str, pos: &Vector2D<i32>) {
        let image = self.images.borrow();
        let rect = VRect::new(5, 5, 40, 40);

        if let Some(image) = image.get(sprite_name) {
            self.context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &image,
                    rect.x as f64,
                    rect.y as f64,
                    rect.w as f64,
                    rect.h as f64,
                    pos.x as f64,
                    pos.y as f64,
                    rect.w as f64,
                    rect.h as f64,
                )
                .expect("draw_image_with... failed");
        }
    }

    fn draw_bg(&mut self, path: &str, is_fullscreen: bool) {
        self.context.set_fill_style(&JsValue::from(format!("rgb({},{},{})", 255, 0, 0)));
    }
}

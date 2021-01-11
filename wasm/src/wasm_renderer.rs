use teki_common::traits::Renderer;
use vector2d::Vector2D;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlImageElement, Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub struct WasmRenderer {
    canvas: HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
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

        Self { canvas, context }
    }
}

impl Renderer for WasmRenderer {
    fn load_sprite(&mut self, filename: &str, x: i32, y: i32, w: u32, h: u32) {}
    fn set_draw_gradient(&mut self) {}
    fn clear(&mut self) {}
    fn draw_str(&mut self, path: &str, x: i32, y: i32, text: &str, r: u8, g: u8, b: u8) {}
    fn draw_sprite(&mut self, sprite_name: &str, pos: &Vector2D<i32>) {}
    fn draw_bg(&mut self, path: &str, is_fullscreen: bool) {}
}

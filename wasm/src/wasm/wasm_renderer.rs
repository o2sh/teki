use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use teki_common::traits::Renderer;
use teki_common::utils::{consts::*, SpriteSheet};
use vector2d::Vector2D;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlCanvasElement, HtmlImageElement, Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub struct WasmRenderer {
    canvas: HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    images: Rc<RefCell<HashMap<String, HtmlImageElement>>>,
    sprite_sheet: Rc<RefCell<SpriteSheet>>,
    scrolling_offset: i32,
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

        Self {
            canvas,
            context,
            images: Rc::new(RefCell::new(HashMap::new())),
            sprite_sheet: Rc::new(RefCell::new(SpriteSheet::default())),
            scrolling_offset: 0,
        }
    }
}

impl Renderer for WasmRenderer {
    fn load_sprite_sheet(&mut self, filename: &str) {
        let filename = String::from(filename);
        let sprite_sheet = self.sprite_sheet.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match request(filename).await {
                Ok(text) => {
                    sprite_sheet.borrow_mut().load_sprite_sheet(&text);
                }
                Err(error) => {
                    web_sys::console::error_1(&format!("error: {}", &error).into());
                }
            }
        });
    }

    fn load_textures(&mut self, base_path: &str, filenames: &[&str]) {
        for &filename in filenames.iter() {
            let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

            let path: String = format!("{}/{}", base_path, filename);
            let basename = String::from(Path::new(filename).file_stem().unwrap().to_str().unwrap());
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
    }

    fn clear(&mut self) {
        self.context.fill_rect(0.0, 0.0, self.canvas.width() as f64, self.canvas.height() as f64)
    }

    fn draw_str(
        &mut self,
        _: &str,
        x: i32,
        y: i32,
        size: u32,
        text: &str,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        bold: bool,
    ) {
        self.context.save();
        self.context.set_global_alpha(a as f64 / 255.0);
        self.set_draw_color(r, g, b);
        let b = if bold { "bold " } else { "" };
        self.context.set_font(&format!("{}{}px Arial", b, size));
        self.context
            .fill_text_with_max_width(text, x as f64, y as f64 + 20.0, self.canvas.width() as f64)
            .expect("draw_image_with... failed");

        self.context.restore();
    }
    fn draw_sprite(&mut self, sprite_name: &str, pos: &Vector2D<i32>) {
        let sprite_sheet = self.sprite_sheet.borrow();
        let (sheet, tex_name) = sprite_sheet.get(sprite_name).expect("No sprite_sheet");
        let image = self.images.borrow();
        if let Some(image) = image.get(tex_name) {
            self.context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &image,
                    sheet.frame.x as f64,
                    sheet.frame.y as f64,
                    sheet.frame.w as f64,
                    sheet.frame.h as f64,
                    pos.x as f64,
                    pos.y as f64,
                    sheet.frame.w as f64,
                    sheet.frame.h as f64,
                )
                .expect("draw_image_with... failed");
        }
    }

    fn draw_texture(&mut self, tex_name: &str, width: i32, height: i32) {
        let image = self.images.borrow();
        if let Some(image) = image.get(tex_name) {
            self.context
                .draw_image_with_html_image_element_and_dw_and_dh(
                    &image,
                    0.0,
                    0.0,
                    width as f64,
                    height as f64,
                )
                .expect("draw_image_with... failed");
        }
    }

    fn set_draw_color(&mut self, r: u8, g: u8, b: u8) {
        self.context.set_fill_style(&JsValue::from(format!("rgb({},{},{})", r, g, b)));
    }

    fn draw_scrolling_bg(&mut self, sprite_name: &str, width: i32, height: i32) {
        let sprite_sheet = self.sprite_sheet.borrow_mut();
        let (sheet, tex_name) = sprite_sheet.get(sprite_name).expect("No sprite_sheet");
        self.scrolling_offset -= 1 * SCROLLING_BG_VEL;
        if self.scrolling_offset < -height {
            self.scrolling_offset = 0;
        }

        let image = self.images.borrow();
        if let Some(image) = image.get(tex_name) {
            self.context.save();
            self.context.set_global_alpha(BG_ALPHA as f64 / 255.0);
            self.context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &image,
                    sheet.frame.x as f64,
                    sheet.frame.y as f64,
                    sheet.frame.w as f64,
                    sheet.frame.h as f64,
                    0.0,
                    self.scrolling_offset as f64,
                    width as f64,
                    sheet.frame.h as f64,
                )
                .expect("draw_image_with... failed");

            self.context.restore();
        }
    }

    fn draw_vertical_separation(&mut self, width: i32, height: i32) {
        self.context.set_fill_style(&JsValue::from(format!("rgb({},{},{})", 255, 255, 255)));
        self.context.fill_rect(width as f64, 0.0, 2.0, height as f64)
    }

    fn draw_rect(
        &mut self,
        pos: &Vector2D<i32>,
        width: i32,
        height: i32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) {
        self.context.save();
        self.context.set_global_alpha(a as f64 / 255.0);
        self.context.set_fill_style(&JsValue::from(format!("rgba({},{},{})", r, g, b)));
        self.context.fill_rect(pos.x as f64, pos.y as f64, width as f64, height as f64);
        self.context.restore();
    }
}

async fn request(url: String) -> Result<String, String> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)
        .or_else(|_| Err(String::from("request init failed")))?;

    request
        .headers()
        .set("Accept", "text/plain")
        .or_else(|_| Err(String::from("request header error")))?;

    let window = web_sys::window().unwrap();
    let resp_value =
        JsFuture::from(window.fetch_with_request(&request)).await.expect("future failed");

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let text =
        JsFuture::from(resp.text().expect("text")).await.expect("await").as_string().unwrap();

    Ok(text)
}

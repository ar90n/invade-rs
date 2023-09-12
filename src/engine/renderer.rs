use anyhow::Result;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::browser;
use super::geometry::Rect;

pub trait Renderer {
    fn clear(&self, rect: &Rect);
    fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect);
    fn draw_rect(&self, rect: &Rect);
}

pub struct CanvasRenderer {
    context: CanvasRenderingContext2d,
}

impl CanvasRenderer {
    pub fn new() -> Result<Self> {
        let context = browser::context()?;
        Ok(Self { context })
    }
}

impl Renderer for CanvasRenderer {
    fn clear(&self, rect: &Rect) {
        self.context.set_fill_style(&JsValue::from_str("black"));
        self.context.fill_rect(
            rect.x().into(),
            rect.y().into(),
            rect.width().into(),
            rect.height().into(),
        );
    }

    fn draw_image(&self, image: &HtmlImageElement, source: &Rect, destination: &Rect) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                source.x().into(),
                source.y().into(),
                source.width().into(),
                source.height().into(),
                destination.x().into(),
                destination.y().into(),
                destination.width().into(),
                destination.height().into(),
            )
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }

    fn draw_rect(&self, rect: &Rect) {
        self.context.stroke_rect(
            rect.x().into(),
            rect.y().into(),
            rect.width().into(),
            rect.height().into(),
        );
    }
}

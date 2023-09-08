use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::geometry::{Point, Rect};

pub trait Renderer {
    fn clear(&self, rect: &Rect);
    fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect);
    fn draw_entire_image(&self, image: &HtmlImageElement, position: &Point);
    fn draw_rect(&self, rect: &Rect);
}

pub struct CanvasRenderer {
    context: CanvasRenderingContext2d,
}

impl CanvasRenderer {
    pub fn new(context: CanvasRenderingContext2d) -> Self {
        Self { context }
    }
}

impl Renderer for CanvasRenderer {
    fn clear(&self, rect: &Rect) {
        self.context.clear_rect(
            rect.x().into(),
            rect.y().into(),
            rect.width.into(),
            rect.height.into(),
        )
    }

    fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                frame.x().into(),
                frame.y().into(),
                frame.width.into(),
                frame.height.into(),
                destination.x().into(),
                destination.y().into(),
                destination.width.into(),
                destination.height.into(),
            )
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }

    fn draw_entire_image(&self, image: &HtmlImageElement, position: &Point) {
        self.context
            .draw_image_with_html_image_element(image, position.x.into(), position.y.into())
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }

    fn draw_rect(&self, rect: &Rect) {
        self.context.stroke_rect(
            rect.origin.x.into(),
            rect.origin.y.into(),
            rect.width.into(),
            rect.height.into(),
        );
    }
}

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use anyhow::{Result, anyhow};
use serde::Deserialize;
use futures::channel::oneshot::channel;
use web_sys::HtmlImageElement;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::geometry::{Rect, Point};
use super::renderer::Renderer;
use super::browser;

#[derive(Deserialize, Clone)]
pub struct SheetRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub frame: SheetRect,
    pub sprite_source_size: SheetRect,
}

#[derive(Deserialize, Clone)]
pub struct Sheet {
    pub frames: HashMap<String, Cell>,
}

pub struct SpriteSheet {
    sheet: Sheet,
    image: HtmlImageElement,
}

impl SpriteSheet {
    pub fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
        Self { sheet, image }
    }

    pub fn cell(&self, name: &str) -> Option<&Cell> {
        self.sheet.frames.get(name)
    }

    pub fn draw(&self, renderer: &impl Renderer, source: &Rect, destination: &Rect) {
        renderer.draw_image(&self.image, source, destination)
    }
}

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;

    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);
    let success_callback = Closure::once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(()));
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = Closure::once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            error_tx.send(Err(anyhow!("Error Loading Image: {:#?}", err)));
        }
    });

    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);

    complete_rx.await??;

    Ok(image)
}

pub struct Image {
    element: HtmlImageElement,
    bounding_box: Rect,
}

impl Image {
    pub fn new(element: HtmlImageElement, position: Point) -> Self {
        let bounding_box = Rect::new(position, element.width() as i16, element.height() as i16);
        Self {
            element,
            bounding_box,
        }
    }

    pub fn bounding_box(&self) -> &Rect {
        &self.bounding_box
    }

    pub fn set_x(&mut self, x: i16) {
        self.bounding_box.set_x(x);
    }

    pub fn right(&self) -> i16 {
        self.bounding_box.right()
    }

    pub fn move_horizontally(&mut self, distance: i16) {
        self.set_x(self.bounding_box.x() + distance)
    }

    pub fn draw(&self, renderer: &impl Renderer) {
        renderer.draw_entire_image(&self.element, &self.bounding_box.origin);
        renderer.draw_rect(self.bounding_box());
    }
}

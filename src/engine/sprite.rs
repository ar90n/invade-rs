use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use anyhow::{anyhow, Result};
use futures::channel::oneshot::channel;
use serde::Deserialize;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

use super::browser;
use super::geometry::{Point, Rect, Shape};
use super::renderer::Renderer;

#[derive(Deserialize, Clone)]
struct SheetRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

impl Into<Rect> for SheetRect {
    fn into(self) -> Rect {
        Rect::new_from_x_y_w_h(self.x, self.y, self.w, self.h)
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    frame: SheetRect,
}

impl Cell {
    pub fn shape(&self) -> Shape {
        Shape {
            width: self.frame.w,
            height: self.frame.h,
        }
    }
}

#[derive(Deserialize, Clone)]
struct Sheet {
    pub frames: HashMap<String, Cell>,
}

pub struct SpriteSheet {
    sheet: Sheet,
    image: HtmlImageElement,
}

impl SpriteSheet {
    pub async fn load(json_path: &str, png_path: &str) -> Result<SpriteSheet> {
        let json = browser::fetch_json(json_path).await?;
        let sheet: Sheet = json.into_serde()?;
        let image = load_image_element(png_path).await?;

        Ok(SpriteSheet::new(sheet, image))
    }

    pub fn cell(&self, name: &str) -> Option<&Cell> {
        self.sheet.frames.get(name)
    }

    pub fn draw(&self, renderer: &impl Renderer, cell: &Cell, destination: &Point) {
        let source: Rect = cell.clone().frame.into();
        let destination = Rect::new(destination.clone(), cell.shape());

        renderer.draw_image(&self.image, &source, &destination)
    }

    fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
        Self { sheet, image }
    }
}

async fn load_image_element(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;

    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);
    let success_callback = Closure::once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(())).expect("Failed to send success");
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = Closure::once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            error_tx
                .send(Err(anyhow!("Error Loading Image: {:#?}", err)))
                .expect("Failed to send err");
        }
    });

    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);

    complete_rx.await??;

    Ok(image)
}

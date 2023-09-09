#[macro_use]
mod engine;
mod invade_rs;

use wasm_bindgen::prelude::*;

use crate::engine::{event, renderer, GameLoop};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    engine::browser::spawn_local(async move {
        let game = invade_rs::InvadeRs::new();
        let renderer = renderer::CanvasRenderer::new().expect("Could not create renderer");
        let event_source = event::BrowserEventSource::new().expect("Could not create event source");

        GameLoop::start(game, renderer, event_source)
            .await
            .expect("Could not start game loop");
    });
    Ok(())
}

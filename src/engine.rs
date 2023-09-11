pub mod browser;
pub mod event;
pub mod geometry;
pub mod renderer;
pub mod sequence;
pub mod sound;
pub mod sprite;
pub mod ui;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use std::cell::RefCell;
use std::rc::Rc;

use self::event::{Event, EventSource};
use self::renderer::Renderer;

type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;


pub struct DrawCommand(pub i16, pub Box<dyn Fn(&dyn Renderer)>);

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&mut self) -> Result<()>;
    fn update(&mut self, delta: f32, events: &[Event]) -> Result<()>;
    fn draw(&self) -> Vec<DrawCommand>;
}

pub struct GameLoop<G: Game + 'static, R: renderer::Renderer + 'static, E: EventSource + 'static> {
    game: G,
    renderer: R,
    event_source: E,
    last_frame: f64,
    accumulated_delta: f32,
}

impl<G: Game + 'static, R: Renderer + 'static, E: EventSource + 'static> GameLoop<G, R, E> {
    fn new(game: G, renderer: R, event_source: E, last_frame: f64) -> Self {
        Self {
            game,
            renderer,
            event_source,
            last_frame,
            accumulated_delta: 0.0,
        }
    }

    pub async fn start(game: G, renderer: R, event_source: E) -> Result<()> {
        let mut game_loop = {
            let last_frame = browser::now()?;
            Self::new(game, renderer, event_source, last_frame)
        };

        game_loop.initialize().await;

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            let events = game_loop.collect_events();
            game_loop.update(perf, &events);
            game_loop.render();

            browser::request_animation_frame(f.borrow().as_ref().unwrap()).expect("Loop failed");
        }));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;
        Ok(())
    }

    async fn initialize(&mut self) {
        self.game.initialize().await;
    }

    fn collect_events(&mut self) -> Vec<Event> {
        let mut events = vec![];
        while let Some(evt) = self.event_source.try_next() {
            events.push(evt);
        }
        events
    }

    fn update(&mut self, perf: f64, events: &[Event]) {
        let delta = (perf - self.last_frame) as f32;

        self.game.update(delta, &events);

        self.accumulated_delta += delta;
        self.last_frame = perf;
    }

    fn render(&self) {
        let mut drawables = self.game.draw();
        drawables.sort_by_key(|d| d.0);
        drawables.iter().for_each(|d| d.1(&self.renderer));
    }
}

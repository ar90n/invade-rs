use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use std::cell::RefCell;
use std::rc::Rc;

pub mod browser;
pub mod event;
pub mod geometry;
pub mod renderer;
pub mod sound;
pub mod sprite;
pub mod ui;

type SharedLoopClosure = Rc<RefCell<Option<browser::LoopClosure>>>;

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&mut self);
    fn update(&mut self, kyestate: &event::KeyState);
    fn draw(&self, renderer: &dyn renderer::Renderer);
}

const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;

pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f32,
}

impl GameLoop {
    pub async fn start(mut game: impl Game + 'static) -> Result<()> {
        let mut keyevent_receiver = event::prepare_input()?;
        let renderer = renderer::CanvasRenderer::new(browser::context()?);
        let mut keystate = event::KeyState::new();

        let mut game_loop = GameLoop {
            last_frame: browser::now()?,
            accumulated_delta: 0.0,
        };

        game.initialize().await;

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            event::process_input(&mut keystate, &mut keyevent_receiver);
            game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&keystate);
                game_loop.accumulated_delta -= FRAME_SIZE;
            }

            game_loop.last_frame = perf;
            game.draw(&renderer);

            browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;
        Ok(())
    }
}

use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;

use crate::engine::event::Event;
use crate::engine::geometry::Rect;
use crate::engine::renderer::Renderer;
use crate::engine::sprite;
use crate::engine::sprite::SpriteSheet;
use crate::engine::{DrawCommand, Game};

use self::fsm::StateMachineRunner;
use self::game_state::{out_game::OutGame, GameStateMachine};

mod beam;
mod character;
mod ferris;
mod fsm;
mod game_state;
mod missile;
mod shield;
mod ship;
mod turbo_fish;
mod wall;

const SCREEN_RECT: Rect = Rect::new_from_x_y_w_h(0, 0, 600, 600);

pub struct InvadeRs {
    runner: StateMachineRunner<Event, GameStateMachine>,
}

impl InvadeRs {
    pub fn new() -> Self {
        Self {
            runner: StateMachineRunner::new(),
        }
    }

    async fn load_sprite_sheet(&mut self) -> Result<Rc<SpriteSheet>> {
        sprite::SpriteSheet::load("texture.json", "texture.png")
            .await
            .map(|sprite_sheet| Rc::new(sprite_sheet))
    }
}

impl Default for InvadeRs {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Game for InvadeRs {
    async fn initialize(&mut self) -> Result<()> {
        let sprite_sheet = self.load_sprite_sheet().await?;

        self.runner.transition(OutGame::new(sprite_sheet).into())?;
        Ok(())
    }

    fn update(&mut self, delta: f32, events: &[Event]) -> Result<()> {
        self.runner.update(delta, events)
    }

    fn draw(&self) -> Vec<DrawCommand> {
        let clear_command = DrawCommand(
            0,
            Box::new(|renderer: &dyn Renderer| {
                renderer.clear(&SCREEN_RECT);
            }),
        );

        let mut draw_commands = vec![clear_command];
        draw_commands.append(self.runner.state.draw().as_mut());
        draw_commands
    }
}

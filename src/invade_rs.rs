use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;

use crate::engine::event::Event;
use crate::engine::geometry::{Point, Rect};
use crate::engine::renderer::Renderer;
use crate::engine::sprite;
use crate::engine::sprite::SpriteSheet;
use crate::engine::{DrawCommand, Game};

use self::character::GameCharacter;
use self::ferris::{Ferris, FerrisColor};
use self::fsm::StateMachineRunner;
use self::game_state::{in_game::InGame, out_game::OutGame, GameStateMachine};
use self::shield::ShieldElement;
use self::ship::Ship;

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

    fn spawn_ferris_fleet(
        &self,
        sprite_sheet: &Rc<SpriteSheet>,
        screen_width: i16,
    ) -> Vec<Rc<RefCell<GameCharacter>>> {
        const FLEET_COLS: i16 = 9;
        const FLEET_ROWS: i16 = 2 * 3;
        const Y_ORIGIN: i16 = 100;
        const MARGIN: i16 = 10;

        let ferris_shape = Ferris::get_shape(sprite_sheet);
        let x_origin = (screen_width - FLEET_COLS * (ferris_shape.width + MARGIN)) / 2;
        let colors = vec![FerrisColor::Magenta, FerrisColor::Green, FerrisColor::Blue];

        let mut characters = vec![];
        for row in 0..FLEET_ROWS {
            let color = colors[(row / 2) as usize];
            let y = Y_ORIGIN + row * (ferris_shape.height + MARGIN);
            for col in 0..FLEET_COLS {
                let x = x_origin + (MARGIN / 2) + col * (ferris_shape.width + MARGIN);
                let position = Point { x, y };
                let ferris = Ferris::new(sprite_sheet.clone(), position, color);
                characters.push(Rc::new(RefCell::new(ferris.into())));
            }
        }

        characters
    }

    fn spawn_aligned_shields(
        &self,
        sprite_sheet: &Rc<SpriteSheet>,
        screen_width: i16,
    ) -> Vec<Rc<RefCell<GameCharacter>>> {
        const SHIELD_NUM: i16 = 4;
        const Y_ORIGIN: i16 = 490;

        let shield_element_shape = ShieldElement::get_shape(sprite_sheet);
        let shield_width = 4 * shield_element_shape.width;
        let margin = (screen_width - SHIELD_NUM * shield_width) / (2 * SHIELD_NUM);

        let mut characters = vec![];
        for i in 0..SHIELD_NUM {
            let x = margin + i * (shield_width + 2 * margin);
            let position = Point { x, y: Y_ORIGIN };
            shield::create_shield(sprite_sheet.clone(), &position)
                .into_iter()
                .for_each(|c| characters.push(Rc::new(RefCell::new(c.into()))));
        }

        characters
    }

    fn spawn_ship(&self, sprite_sheet: &Rc<SpriteSheet>, screen_width: i16) -> Rc<RefCell<Ship>> {
        const Y_ORIGIN: i16 = 560;

        let ship_shape = ship::Ship::get_shape(sprite_sheet);
        let x_origin = (screen_width - ship_shape.width) / 2;

        let position = Point {
            x: x_origin,
            y: Y_ORIGIN,
        };
        let ship = ship::Ship::new(sprite_sheet.clone(), position);
        Rc::new(RefCell::new(ship.into()))
    }

    fn spawn_walls(
        &self,
        screen_width: i16,
        screen_height: i16,
    ) -> Vec<Rc<RefCell<GameCharacter>>> {
        let left_wall = wall::Wall::new(
            wall::WallType::Left,
            Rect::new_from_x_y_w_h(0, 0, 1, screen_height),
        );
        let right_wall = wall::Wall::new(
            wall::WallType::Right,
            Rect::new_from_x_y_w_h(screen_width - 1, 0, 1, screen_height),
        );

        vec![
            Rc::new(RefCell::new(left_wall.into())),
            Rc::new(RefCell::new(right_wall.into())),
        ]
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
        let characters = {
            let mut characters = vec![];
            characters.append(&mut self.spawn_ferris_fleet(&sprite_sheet, SCREEN_RECT.width()));
            characters.append(&mut self.spawn_aligned_shields(&sprite_sheet, SCREEN_RECT.width()));
            characters.append(&mut self.spawn_walls(SCREEN_RECT.width(), SCREEN_RECT.height()));
            characters
        };
        let player = self.spawn_ship(&sprite_sheet, SCREEN_RECT.width());

        self.runner
            //.transition(OutGame::new(sprite_sheet, characters).into())?;
            .transition(InGame::new(sprite_sheet, characters, player).into())?;
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

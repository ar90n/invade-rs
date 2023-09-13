use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;

use crate::engine::event::Event;
use crate::engine::geometry::{Point, Rect};
use crate::engine::sprite::SpriteSheet;
use crate::engine::DrawCommand;

use super::super::character::GameCharacter;
use super::super::ferris::{Ferris, FerrisColor};
use super::super::fsm::State;
use super::super::shield::{create_shield, ShieldElement};
use super::super::ship::Ship;
use super::super::wall::{Wall, WallType};
use super::in_game::InGame;
use super::GameStateMachine;

const SCREEN_RECT: Rect = Rect::new_from_x_y_w_h(0, 0, 600, 600);

#[derive(Clone)]
pub struct OutGame {
    sprite_sheet: Rc<SpriteSheet>,
    characters: Vec<Rc<RefCell<GameCharacter>>>,
    player: Rc<RefCell<Ship>>,
}

impl OutGame {
    pub fn new(sprite_sheet: Rc<SpriteSheet>) -> Self {
        let characters = {
            let mut characters = vec![];
            characters.append(&mut Self::spawn_ferris_fleet(
                &sprite_sheet,
                SCREEN_RECT.width(),
            ));
            characters.append(&mut Self::spawn_aligned_shields(
                &sprite_sheet,
                SCREEN_RECT.width(),
            ));
            characters.append(&mut Self::spawn_walls(
                SCREEN_RECT.width(),
                SCREEN_RECT.height(),
            ));
            characters
        };
        let player = Self::spawn_ship(&sprite_sheet, SCREEN_RECT.width());

        Self {
            sprite_sheet,
            characters,
            player,
        }
    }

    pub fn draw(&self) -> Vec<DrawCommand> {
        let mut draw_commands = vec![];
        draw_commands.append(&mut self.characters.iter().map(|c| c.borrow().draw()).collect());
        draw_commands.push(self.player.borrow().draw());
        draw_commands.into_iter().flatten().collect()
    }

    fn spawn_ferris_fleet(
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
            create_shield(sprite_sheet.clone(), &position)
                .into_iter()
                .for_each(|c| characters.push(Rc::new(RefCell::new(c.into()))));
        }

        characters
    }

    fn spawn_ship(sprite_sheet: &Rc<SpriteSheet>, screen_width: i16) -> Rc<RefCell<Ship>> {
        const Y_ORIGIN: i16 = 560;

        let ship_shape = Ship::get_shape(sprite_sheet);
        let x_origin = (screen_width - ship_shape.width) / 2;

        let position = Point {
            x: x_origin,
            y: Y_ORIGIN,
        };
        let ship = Ship::new(sprite_sheet.clone(), position);
        Rc::new(RefCell::new(ship))
    }

    fn spawn_walls(screen_width: i16, screen_height: i16) -> Vec<Rc<RefCell<GameCharacter>>> {
        let left_wall = Wall::new(
            WallType::Left,
            Rect::new_from_x_y_w_h(0, 0, 1, screen_height),
        );
        let right_wall = Wall::new(
            WallType::Right,
            Rect::new_from_x_y_w_h(screen_width - 1, 0, 1, screen_height),
        );

        vec![
            Rc::new(RefCell::new(left_wall.into())),
            Rc::new(RefCell::new(right_wall.into())),
        ]
    }
}

impl State<Event, GameStateMachine> for OutGame {
    fn update(&self, _delta_ms: f32, _events: &[Event]) -> GameStateMachine {
        GameStateMachine::InGame(InGame::new(
            self.sprite_sheet.clone(),
            self.characters.clone(),
            self.player.clone(),
        ))
    }

    fn on_enter(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_exit(&mut self) -> Result<()> {
        Ok(())
    }
}

impl From<OutGame> for GameStateMachine {
    fn from(val: OutGame) -> Self {
        GameStateMachine::OutGame(val)
    }
}

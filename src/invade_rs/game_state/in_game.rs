use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;


use crate::engine::event::Event;
use crate::engine::geometry::{Point, Rect};
use crate::engine::sprite::SpriteSheet;
use crate::engine::DrawCommand;

use super::super::character::{GameCharacter, GameCommand};
use super::super::fsm::State;
use super::super::ship::Ship;
use super::super::turbo_fish;
use super::out_game::OutGame;
use super::GameStateMachine;

const SCREEN_RECT: Rect = Rect::new_from_x_y_w_h(0, 0, 600, 600);

#[derive(Clone)]
pub struct InGame {
    sprite_sheet: Rc<SpriteSheet>,
    pub characters: Vec<Rc<RefCell<GameCharacter>>>,
    pub player: Rc<RefCell<Ship>>,
    is_game_over: bool,
}

impl InGame {
    fn apply_event(&self, event: &Event) {
        match event {
            Event::KeyDown(key) => match key.as_str() {
                "ArrowRight" => self.player.borrow_mut().move_right(),
                "ArrowLeft" => self.player.borrow_mut().move_left(),
                "Space" => self.player.borrow_mut().shot(),
                _ => {}
            },
            Event::KeyUp(key) => match key.as_str() {
                "ArrowRight" => self.player.borrow_mut().stop(),
                "ArrowLeft" => self.player.borrow_mut().stop(),
                "Space" => self.player.borrow_mut().reload(),
                _ => {}
            },
        }
    }

    fn update_game(&self, delta: f32) -> Vec<GameCommand> {
        const TURBO_FISH_APPEAR_PROBABILITY: f32 = 0.001;

        let mut commands = vec![];
        for c in self.characters.iter() {
            let mut c = c.borrow_mut();

            let cur_visible = c.bounding_box().intersects(&SCREEN_RECT);
            if let Some(command) = c.update(delta) {
                commands.push(command);
            }
            let next_visible = c.bounding_box().intersects(&SCREEN_RECT);

            if cur_visible && !next_visible {
                if let Some(command) = c.on_exit_screen() {
                    commands.push(command);
                }
            }
        }

        for c in self.characters.iter() {
            let c = c.borrow();
            for other in self.characters.iter() {
                let other = other.borrow();
                if c.id() == other.id() {
                    continue;
                }

                if c.bounding_box().intersects(&other.bounding_box()) {
                    if let Some(command) = c.on_collide(&other) {
                        commands.push(command);
                    }
                }
            }

            let player = self.player.borrow();
            if c.bounding_box().intersects(&player.bounding_box()) {
                if let Some(command) = c.on_collide(&player.clone().into()) {
                    commands.push(command);
                }
                if let Some(command) = player.on_collide(&c) {
                    commands.push(command);
                }
            }
        }

        if let Some(command) = self.player.borrow_mut().update(delta) {
            commands.push(command);
        }

        if rand::random::<f32>() < TURBO_FISH_APPEAR_PROBABILITY {
            commands.push(self.create_spawn_turbo_fish_command(&self.sprite_sheet));
        }
        commands
    }

    fn apply_command(&mut self, command: GameCommand) {
        match command {
            GameCommand::SpawnCharacter(new_character) => {
                self.characters.push(Rc::new(RefCell::new(new_character)));
            }
            GameCommand::DestroyCharacter(id) => {
                self.characters.retain(|c| c.borrow().id() != &id);
            }
            GameCommand::TurnFerris => {
                for c in self.characters.iter() {
                    let mut c = c.borrow_mut();
                    if let GameCharacter::Ferris(ferris) = &mut *c {
                        ferris.turn();
                    }
                }
            }
            GameCommand::DestroyPlayer => {
                self.is_game_over = true;
            }
            _ => {}
        }
    }

    pub fn new(
        sprite_sheet: Rc<SpriteSheet>,
        characters: Vec<Rc<RefCell<GameCharacter>>>,
        player: Rc<RefCell<Ship>>,
    ) -> Self {
        Self {
            sprite_sheet,
            characters,
            player,
            is_game_over: false,
        }
    }
    pub fn draw(&self) -> Vec<DrawCommand> {
        let mut draw_commands = vec![];
        draw_commands.append(&mut self.characters.iter().map(|c| c.borrow().draw()).collect());
        draw_commands.push(self.player.borrow().draw());
        draw_commands.into_iter().flatten().collect()
    }

    fn create_spawn_turbo_fish_command(&self, sprite_sheet: &Rc<SpriteSheet>) -> GameCommand {
        const Y_ORIGIN: i16 = 50;

        let ship_shape = turbo_fish::TurboFish::get_shape(sprite_sheet);
        let x_origin = -ship_shape.width;

        let position = Point {
            x: x_origin,
            y: Y_ORIGIN,
        };
        let turbo_fish = turbo_fish::TurboFish::new(sprite_sheet.clone(), position);
        GameCommand::SpawnCharacter(turbo_fish.into())
    }
}

impl State<Event, GameStateMachine> for InGame {
    fn update(&self, delta: f32, events: &[Event]) -> GameStateMachine {
        events.iter().for_each(|event| self.apply_event(event));

        let commands = self.update_game(delta);

        let next_state = {
            let mut next_state = self.clone();
            for c in commands.into_iter() {
                next_state.apply_command(c);
            }
            next_state
        };

        let mut enemy_count = 0;
        for c in self.characters.iter() {
            let c = c.borrow();
            if let GameCharacter::Ferris(_) = &*c {
                enemy_count += 1;
            }
        }

        if enemy_count == 0 {
            return GameStateMachine::OutGame(OutGame::new(self.sprite_sheet.clone()));
        }

        if self.is_game_over {
            return GameStateMachine::OutGame(OutGame::new(self.sprite_sheet.clone()));
        }

        GameStateMachine::InGame(next_state)
    }

    fn on_enter(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_exit(&mut self) -> Result<()> {
        Ok(())
    }
}

impl From<InGame> for GameStateMachine {
    fn from(val: InGame) -> Self {
        GameStateMachine::InGame(val)
    }
}

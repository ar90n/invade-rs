use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;

use crate::engine::browser;
use crate::engine::event::Event;
use crate::engine::sprite;

use super::super::character::GameCharacter;
use super::super::fsm::State;
use super::GameStateMachine;

#[derive(Clone)]
pub struct OutGame {
    sprite_sheet: Rc<sprite::SpriteSheet>,
    pub characters: Vec<Rc<RefCell<GameCharacter>>>,
}

impl OutGame {
    fn new(
        sprite_sheet: Rc<sprite::SpriteSheet>,
        characters: Vec<Rc<RefCell<GameCharacter>>>,
    ) -> Self {
        Self {
            sprite_sheet,
            characters,
        }
    }
}

impl State<Event, GameStateMachine> for OutGame {
    fn update(&self, delta: f32, events: &[Event]) -> GameStateMachine {
        GameStateMachine::OutGame(self.clone())
    }

    fn on_enter(&mut self) -> Result<()> {
        browser::log("enter OutGame");
        Ok(())
    }

    fn on_exit(&mut self) -> Result<()> {
        browser::log("exit OutGame");
        Ok(())
    }
}

impl Into<GameStateMachine> for OutGame {
    fn into(self) -> GameStateMachine {
        GameStateMachine::OutGame(self)
    }
}

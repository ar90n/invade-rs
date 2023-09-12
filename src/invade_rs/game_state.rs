use anyhow::Result;

use crate::engine::event::Event;
use crate::engine::DrawCommand;

use super::fsm::{State, StateMachine};

use self::created::*;
use self::in_game::*;
use self::out_game::*;

pub mod created;
pub mod in_game;
pub mod out_game;

pub enum GameStateMachine {
    Created(Created),
    OutGame(OutGame),
    InGame(InGame),
}

impl GameStateMachine {
    pub fn draw(&self) -> Vec<DrawCommand> {
        match self {
            Self::Created(_) => vec![],
            Self::OutGame(state) => state
                .characters
                .iter()
                .map(|c| c.borrow().draw())
                .filter_map(|c| c)
                .collect(),
            Self::InGame(state) => {
                let mut draw_commands = vec![];
                draw_commands
                    .append(&mut state.characters.iter().map(|c| c.borrow().draw()).collect());
                draw_commands.push(state.player.borrow().draw());
                draw_commands.into_iter().filter_map(|c| c).collect()
            }
        }
    }
}

impl StateMachine<Event> for GameStateMachine {
    fn update(&self, delta: f32, events: &[Event]) -> Self {
        match self {
            Self::Created(state) => state.update(delta, events),
            Self::OutGame(state) => state.update(delta, events),
            Self::InGame(state) => state.update(delta, events),
        }
    }

    fn on_enter(&mut self) -> Result<()> {
        match self {
            Self::Created(state) => state.on_enter(),
            Self::OutGame(state) => state.on_enter(),
            Self::InGame(state) => state.on_enter(),
        }
    }

    fn on_exit(&mut self) -> Result<()> {
        match self {
            Self::Created(state) => state.on_exit(),
            Self::OutGame(state) => state.on_exit(),
            Self::InGame(state) => state.on_exit(),
        }
    }
}

impl Default for GameStateMachine {
    fn default() -> Self {
        Self::Created(Created {})
    }
}

impl PartialEq for GameStateMachine {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Created(_), Self::Created(_)) => true,
            (Self::OutGame(_), Self::OutGame(_)) => true,
            (Self::InGame(_), Self::InGame(_)) => true,
            _ => false,
        }
    }
}

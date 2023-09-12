use anyhow::Result;

use crate::engine::browser;
use crate::engine::event::Event;

use super::super::fsm::State;
use super::GameStateMachine;

#[derive(Clone)]
pub struct Created {}

impl State<Event, GameStateMachine> for Created {
    fn update(&self, _delta_ms: f32, _events: &[Event]) -> GameStateMachine {
        GameStateMachine::Created(self.clone())
    }

    fn on_enter(&mut self) -> Result<()> {
        browser::log("enter Created");
        Ok(())
    }

    fn on_exit(&mut self) -> Result<()> {
        browser::log("exit Created");
        Ok(())
    }
}

impl Into<GameStateMachine> for Created {
    fn into(self) -> GameStateMachine {
        GameStateMachine::Created(self)
    }
}

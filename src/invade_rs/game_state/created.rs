use anyhow::Result;

use crate::engine::event::Event;
use crate::engine::DrawCommand;

use super::super::fsm::State;
use super::GameStateMachine;

#[derive(Clone)]
pub struct Created {}

impl Created {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self) -> Vec<DrawCommand> {
        vec![]
    }
}

impl State<Event, GameStateMachine> for Created {
    fn update(&self, _delta_ms: f32, _events: &[Event]) -> GameStateMachine {
        GameStateMachine::Created(self.clone())
    }

    fn on_enter(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_exit(&mut self) -> Result<()> {
        Ok(())
    }
}

impl From<Created> for GameStateMachine {
    fn from(val: Created) -> Self {
        GameStateMachine::Created(val)
    }
}

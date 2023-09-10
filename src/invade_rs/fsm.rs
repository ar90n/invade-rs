use anyhow::Result;
use async_trait::async_trait;

use super::event::Event;

#[async_trait(?Send)]
pub trait State<SM: StateMachine> {
    fn update(&self, delta: f32, events: &[Event]) -> SM;
    fn on_enter(&mut self) -> Result<()>;
    fn on_exit(&mut self) -> Result<()>;
}

#[async_trait(?Send)]
pub trait StateMachine {
    fn update(&self, delta: f32, events: &[Event]) -> Self;
    fn on_enter(&mut self) -> Result<()>;
    fn on_exit(&mut self) -> Result<()>;
}

pub struct StateMachineRunner<S: StateMachine + Default + PartialEq> {
    pub state: S,
}

impl<S: StateMachine + Default + PartialEq> StateMachineRunner<S> {
    pub fn new() -> Self {
        Self {
            state: S::default(),
        }
    }

    pub fn update(&mut self, delta: f32, events: &[Event]) -> Result<()> {
        let mut next_state = self.state.update(delta, events);
        self.transition(next_state)
    }

    pub fn transition(&mut self, mut next_state: S) -> Result<()> {
        if next_state != self.state {
            self.state.on_exit()?;
            next_state.on_enter()?;
        }
        self.state = next_state;

        Ok(())
    }
}

use std::marker::PhantomData;

use anyhow::Result;

pub trait State<E, SM: StateMachine<E>> {
    fn update(&self, delta: f32, events: &[E]) -> SM;
    fn on_enter(&mut self) -> Result<()>;
    fn on_exit(&mut self) -> Result<()>;
}

pub trait StateMachine<E> {
    fn update(&self, delta: f32, events: &[E]) -> Self;
    fn on_enter(&mut self) -> Result<()>;
    fn on_exit(&mut self) -> Result<()>;
}

pub struct StateMachineRunner<E, S: StateMachine<E> + Default + PartialEq> {
    pub state: S,
    phantom: PhantomData<E>,
}

impl<E, S: StateMachine<E> + Default + PartialEq> StateMachineRunner<E, S> {
    pub fn new() -> Self {
        Self {
            state: S::default(),
            phantom: PhantomData,
        }
    }

    pub fn update(&mut self, delta: f32, events: &[E]) -> Result<()> {
        let next_state = self.state.update(delta, events);
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

use anyhow::Result;
use async_trait::async_trait;

use crate::engine::Game;
use crate::engine::{event, renderer};

pub struct InvadeRs {}

impl InvadeRs {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl Game for InvadeRs {
    async fn initialize(&mut self) {
        todo!()
    }

    fn update(&mut self, kyestate: &event::KeyState) {
        todo!()
    }
    fn draw(&self, renderer: &dyn renderer::Renderer) {
        todo!()
    }
}

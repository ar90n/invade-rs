use uuid::Uuid;

use crate::engine::geometry::Rect;
use crate::engine::DrawCommand;

#[derive(Clone, Debug, PartialEq)]
pub struct Id(String);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4().to_string())
    }
}

pub enum GameCommand {
    SpawnCharacter(Box<dyn Character>),
    DestroyCharacter(Id),
}

pub trait Character {
    fn id(&self) -> &Id;
    fn bounding_box(&self) -> Rect;
    fn update(&mut self, delta: f32) -> Option<GameCommand>;
    fn on_exit_screen(&mut self) -> Option<GameCommand> {
        Some(GameCommand::DestroyCharacter(self.id().clone()))
    }
    fn draw(&self) -> Option<DrawCommand>;
}

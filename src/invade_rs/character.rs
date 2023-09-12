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

pub mod layers {
    pub const BACKGROUND: u8 = 0;
    pub const SHIELD: u8 = 1;
    pub const BEAM: u8 = 2;
    pub const MISSILE: u8 = 3;
    pub const SHIP: u8 = 4;
    pub const ENEMY: u8 = 5;
}
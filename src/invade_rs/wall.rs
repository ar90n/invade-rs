use super::character::{GameCharacter, GameCommand, Id};
use crate::engine::geometry::Rect;
use crate::engine::DrawCommand;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WallType {
    Left,
    Right,
}

pub struct Wall {
    id: Id,
    wall_type: WallType,
    bounding_box: Rect,
}

impl Wall {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn new(wall_type: WallType, bounding_box: Rect) -> Self {
        let id = Id::new();
        Self {
            id,
            wall_type,
            bounding_box,
        }
    }

    pub fn wall_type(&self) -> WallType {
        self.wall_type
    }

    pub fn bounding_box(&self) -> Rect {
        self.bounding_box.clone()
    }

    pub fn update(&mut self, _delta: f32) -> Option<GameCommand> {
        None
    }

    pub fn on_exit_screen(&mut self) -> Option<GameCommand> {
        None
    }

    pub fn on_collide(&self, _other: &GameCharacter) -> Option<GameCommand> {
        None
    }

    pub fn draw(&self) -> Option<DrawCommand> {
        None
    }
}

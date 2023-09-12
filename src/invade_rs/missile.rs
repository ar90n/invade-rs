use std::rc::Rc;

use crate::engine::geometry::{Point, Rect, Shape};
use crate::engine::sprite::{Cell, SpriteSheet};
use crate::engine::DrawCommand;

use super::character::{Character, GameCommand, Id, layers};

#[derive(Clone)]
pub struct Missile {
    id: Id,
    position: Point,
    sprite_sheet: Rc<SpriteSheet>,
    cell: Cell,
    velocity: f32,
}

impl Missile {
    const DEFAULT_VELOCITY: f32 = 120.0 / 1000.0;

    pub fn get_shape(sprite_sheet: &Rc<SpriteSheet>) -> Shape {
        let cell = sprite_sheet
            .cell("beam_orange_1.png")
            .expect("cell not found");
        cell.shape()
    }

    pub fn new(sprite_sheet: Rc<SpriteSheet>, position: Point) -> Self {
        let cell = sprite_sheet
            .cell("beam_orange_1.png")
            .expect("cell not found")
            .clone();

        Self {
            id: Id::new(),
            position,
            sprite_sheet,
            cell,
            velocity: Self::DEFAULT_VELOCITY,
        }
    }
}

impl Character for Missile {
    fn id(&self) -> &Id {
        &self.id
    }

    fn bounding_box(&self) -> Rect {
        Rect::new(self.position.clone(), self.cell.shape())
    }

    fn update(&mut self, delta_ms: f32) -> Option<GameCommand> {
        self.position.y -= (self.velocity * delta_ms).round() as i16;

        None
    }

    fn draw(&self) -> Option<DrawCommand> {
        let cell = self.cell.clone();
        let sprite_sheet = self.sprite_sheet.clone();
        let position = self.position.clone();

        Some(DrawCommand(
            layers::MISSILE,
            Box::new(move |renderer| {
                sprite_sheet.draw(renderer, &cell, &position);
            }),
        ))
    }
}

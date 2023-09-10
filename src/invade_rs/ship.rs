use std::rc::Rc;

use crate::engine::geometry::{Point, Shape};
use crate::engine::sprite::{Cell, SpriteSheet};
use crate::engine::{Character, DrawCommand};

#[derive(Clone)]
pub struct Ship {
    position: Point,
    sprite_sheet: Rc<SpriteSheet>,
    cell: Cell,
}

impl Ship {
    pub fn get_shape(sprite_sheet: &Rc<SpriteSheet>) -> Shape {
        let cell = sprite_sheet
            .cell("rust_logo_orange.png")
            .expect("cell not found");
        cell.shape()
    }

    pub fn new(sprite_sheet: Rc<SpriteSheet>, position: Point) -> Self {
        let cell = sprite_sheet
            .cell("rust_logo_orange.png")
            .expect("cell not found")
            .clone();

        Self {
            position,
            sprite_sheet,
            cell,
        }
    }
}

impl Character for Ship {
    fn update(&mut self, delta: f32) {}

    fn draw(&self) -> DrawCommand {
        let cell = self.cell.clone();
        let sprite_sheet = self.sprite_sheet.clone();
        let position = self.position.clone();

        DrawCommand(
            1,
            Box::new(move |renderer| {
                sprite_sheet.draw(renderer, &cell, &position);
            }),
        )
    }
}

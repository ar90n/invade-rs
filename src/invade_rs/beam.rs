use std::rc::Rc;

use crate::engine::geometry::Rect;
use crate::engine::geometry::{Point, Shape};
use crate::engine::sprite::{Cell, SpriteSheet};
use crate::engine::{DrawCommand, Game};

use super::character::{Character, GameCommand, Id};

pub enum BeamColor {
    Blue,
    Green,
    Magenta,
}

impl Into<&str> for BeamColor {
    fn into(self) -> &'static str {
        match self {
            BeamColor::Blue => "blue",
            BeamColor::Green => "green",
            BeamColor::Magenta => "magenta",
        }
    }
}

#[derive(Clone)]
pub struct Beam {
    id: Id,
    position: Point,
    sprite_sheet: Rc<SpriteSheet>,
    cell: Cell,
    velocity: i16,
}

impl Beam {
    const VELOCITY: i16 = 1;

    pub fn get_shape(sprite_sheet: &Rc<SpriteSheet>) -> Shape {
        let cell = sprite_sheet
            .cell("beam_blue_0.png")
            .expect("cell not found");
        cell.shape()
    }

    pub fn new(sprite_sheet: Rc<SpriteSheet>, position: Point, color: BeamColor) -> Self {
        let cell = sprite_sheet
            .cell(Self::get_cell_name(color).as_str())
            .expect("cell not found")
            .clone();

        Self {
            id: Id::new(),
            position,
            sprite_sheet,
            cell,
            velocity: Self::VELOCITY,
        }
    }

    fn get_cell_name(color: BeamColor) -> String {
        let color_str: &str = color.into();
        let cell_name = format!("beam_{}_0.png", color_str);
        cell_name
    }
}

impl Character for Beam {
    fn id(&self) -> &Id {
        &self.id
    }

    fn bounding_box(&self) -> Rect {
        let shape = self.cell.shape();
        let position = self.position.clone();
        Rect::new(position, shape)
    }

    fn update(&mut self, delta: f32) -> Option<GameCommand> {
        self.position.y += self.velocity;

        None
    }

    fn draw(&self) -> Option<DrawCommand> {
        let cell = self.cell.clone();
        let sprite_sheet = self.sprite_sheet.clone();
        let position = self.position.clone();

        Some(DrawCommand(
            1,
            Box::new(move |renderer| {
                sprite_sheet.draw(renderer, &cell, &position);
            }),
        ))
    }
}

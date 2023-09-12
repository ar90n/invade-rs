use std::rc::Rc;

use crate::engine::geometry::Rect;
use crate::engine::geometry::{Point, Shape};
use crate::engine::sprite::{Cell, SpriteSheet};
use crate::engine::{DrawCommand, Game};

use super::character::{layers, GameCommand, Id, GameCharacter};

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
    velocity: f32,
}

impl Beam {
    const DEFAULT_VELOCITY: f32 = 80.0 / 1000.0;

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
            velocity: Self::DEFAULT_VELOCITY,
        }
    }

    fn get_cell_name(color: BeamColor) -> String {
        let color_str: &str = color.into();
        let cell_name = format!("beam_{}_0.png", color_str);
        cell_name
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn bounding_box(&self) -> Rect {
        let shape = self.cell.shape();
        let position = self.position.clone();
        Rect::new(position, shape)
    }

    pub fn update(&mut self, delta_ms: f32) -> Option<GameCommand> {
        self.position.y += (self.velocity * delta_ms).round() as i16;

        None
    }

    pub fn draw(&self) -> Option<DrawCommand> {
        let cell = self.cell.clone();
        let sprite_sheet = self.sprite_sheet.clone();
        let position = self.position.clone();

        Some(DrawCommand(
            layers::BEAM,
            Box::new(move |renderer| {
                sprite_sheet.draw(renderer, &cell, &position);
            }),
        ))
    }

    pub fn on_exit_screen(&mut self) -> Option<GameCommand> {
        Some(GameCommand::DestroyCharacter(self.id().clone()))
    }

    pub fn on_collide(&self, other: &GameCharacter) -> Option<GameCommand> {
        match other {
            GameCharacter::ShieldElement(_) | GameCharacter::Ship(_) => {
                Some(GameCommand::DestroyCharacter(self.id().clone()))
            }
            _ => None,
        }
    }
}

use std::rc::Rc;

use crate::engine::geometry::{Point, Rect, Shape};
use crate::engine::sequence::{Frame, Sequence};
use crate::engine::sprite::SpriteSheet;
use crate::engine::DrawCommand;

use super::beam::{Beam, BeamColor};
use super::character::{Character, GameCommand, Id};

#[derive(Clone, Copy)]
pub enum FerrisColor {
    Blue,
    Green,
    Magenta,
}

impl Into<&str> for FerrisColor {
    fn into(self) -> &'static str {
        match self {
            FerrisColor::Blue => "blue",
            FerrisColor::Green => "green",
            FerrisColor::Magenta => "magenta",
        }
    }
}

impl Into<BeamColor> for FerrisColor {
    fn into(self) -> BeamColor {
        match self {
            FerrisColor::Blue => BeamColor::Blue,
            FerrisColor::Green => BeamColor::Green,
            FerrisColor::Magenta => BeamColor::Magenta,
        }
    }
}

#[derive(Clone)]
pub struct Ferris {
    id: Id,
    position: Point,
    sprite_sheet: Rc<SpriteSheet>,
    animation: Sequence,
    color: FerrisColor,
}

impl Ferris {
    pub fn get_shape(sprite_sheet: &Rc<SpriteSheet>) -> Shape {
        let cell = sprite_sheet
            .cell("ferris_blue_0.png")
            .expect("cell not found");
        cell.shape()
    }

    pub fn new(sprite_sheet: Rc<SpriteSheet>, position: Point, color: FerrisColor) -> Self {
        let animation = Self::new_animation(color);

        Self {
            id: Id::new(),
            position,
            sprite_sheet,
            animation,
            color,
        }
    }

    fn get_beam_spawn_point(&self) -> Point {
        let cell = self
            .sprite_sheet
            .cell(self.animation.current_frame_cell_name())
            .expect("cell not found");
        let ferris_shape = cell.shape();
        let beam_shape = Beam::get_shape(&self.sprite_sheet);

        Point {
            x: self.position.x + ferris_shape.width / 2 - beam_shape.width / 2,
            y: self.position.y + ferris_shape.height,
        }
    }

    fn new_animation(color: FerrisColor) -> Sequence {
        const FERRIS_ANIMATION_FRAMES: usize = 4;

        let frames = (0..=FERRIS_ANIMATION_FRAMES).into_iter().map(|i| {
            let color_str: &str = color.into();
            let cell_name = format!("ferris_{}_{}.png", color_str, i);
            let duration = 150.0;
            Frame::new(cell_name, duration)
        });
        Sequence::new(frames.collect())
    }
}

impl Character for Ferris {
    fn id(&self) -> &Id {
        &self.id
    }

    fn bounding_box(&self) -> Rect {
        let cell = self
            .sprite_sheet
            .cell(self.animation.current_frame_cell_name())
            .expect("cell not found");
        let shape = cell.shape();
        let position = self.position.clone();

        Rect::new(position, shape)
    }

    fn update(&mut self, delta: f32) -> Option<GameCommand> {
        const SPAWN_BEAM_RATIO: f32 = 0.00010;

        self.animation.update(delta);
        if rand::random::<f32>() < SPAWN_BEAM_RATIO {
            let beam = Beam::new(
                self.sprite_sheet.clone(),
                self.get_beam_spawn_point(),
                self.color.into(),
            );
            Some(GameCommand::SpawnCharacter(Box::new(beam)))
        } else {
            None
        }
    }

    fn draw(&self) -> Option<DrawCommand> {
        let cell = self
            .sprite_sheet
            .cell(self.animation.current_frame_cell_name())
            .expect("cell not found")
            .clone();
        let sprite_sheet = self.sprite_sheet.clone();
        let position = self.position.clone();

        Some(DrawCommand(
            2,
            Box::new(move |renderer| {
                sprite_sheet.draw(renderer, &cell, &position);
            }),
        ))
    }
}

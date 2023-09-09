use std::rc::Rc;

use crate::engine::geometry::{Point, Rect};
use crate::engine::renderer::Renderer;
use crate::engine::sequence::{Frame, Sequence};
use crate::engine::sprite::SpriteSheet;
use crate::engine::{Character, Drawable};

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

pub struct Ferris {
    position: Point,
    sprite_sheet: Rc<SpriteSheet>,
    animation: Sequence,
}

impl Ferris {
    pub fn new(sprite_sheet: Rc<SpriteSheet>, position: Point, color: FerrisColor) -> Self {
        let animation = Self::new_animation(color);

        Self {
            position,
            sprite_sheet,
            animation,
        }
    }

    fn new_animation(color: FerrisColor) -> Sequence {
        const FERRIS_ANIMATION_FRAMES: usize = 4;

        let frames = (0..=FERRIS_ANIMATION_FRAMES).into_iter().map(|i| {
            let color_str: &str = color.into();
            let cell_name = format!("ferris_{}_{}.png", color_str, i);
            let duration = 250.0;
            Frame::new(cell_name, duration)
        });
        Sequence::new(frames.collect())
    }
}

impl Character for Ferris {
    fn update(&mut self, delta: f32) {
        self.animation.update(delta);
    }
}

impl Drawable for Ferris {
    fn draw(&self, renderer: &impl Renderer) {
        if let Some(cell) = self
            .sprite_sheet
            .cell(self.animation.current_frame_cell_name())
        {
            self.sprite_sheet.draw(renderer, &cell, &self.position);
        }
    }
}

use std::rc::Rc;

use crate::engine::geometry::{Point, Rect, Shape};
use crate::engine::sequence::{Frame, Sequence};
use crate::engine::sprite::SpriteSheet;
use crate::engine::{browser, DrawCommand};

use super::character::{Character, GameCommand, Id};

#[derive(Clone)]
pub struct TurboFish {
    id: Id,
    position: Point,
    sprite_sheet: Rc<SpriteSheet>,
    animation: Sequence,
}

impl TurboFish {
    pub fn get_shape(sprite_sheet: &Rc<SpriteSheet>) -> Shape {
        let cell = sprite_sheet
            .cell("ferris_blue_0.png")
            .expect("cell not found");
        cell.shape()
    }

    pub fn new(sprite_sheet: Rc<SpriteSheet>, position: Point) -> Self {
        let animation = Self::new_animation();

        Self {
            id: Id::new(),
            position,
            sprite_sheet,
            animation,
        }
    }

    fn new_animation() -> Sequence {
        const TURBO_FISH_ANIMATION_FRAMES: usize = 2;

        let frames = (0..TURBO_FISH_ANIMATION_FRAMES).into_iter().map(|i| {
            let cell_name = format!("turbo_fish_yellow_{}.png", i);
            let duration = 100.0;
            Frame::new(cell_name, duration)
        });
        Sequence::new(frames.collect())
    }
}

impl Character for TurboFish {
    fn id(&self) -> &Id {
        &self.id
    }
    fn bounding_box(&self) -> Rect {
        Rect::new(self.position.clone(), Self::get_shape(&self.sprite_sheet))
    }

    fn update(&mut self, delta: f32) -> Option<GameCommand> {
        const VELOCITY: i16 = 3;

        self.animation.update(delta);
        self.position.x += VELOCITY;

        None
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
            1,
            Box::new(move |renderer| {
                sprite_sheet.draw(renderer, &cell, &position);
            }),
        ))
    }
}

use std::rc::Rc;

use crate::engine::geometry::{Point, Rect, Shape};
use crate::engine::sequence::{Frame, Sequence};
use crate::engine::sprite::SpriteSheet;
use crate::engine::DrawCommand;

use super::character::{layers, GameCharacter, GameCommand, Id};

#[derive(Clone)]
pub struct TurboFish {
    id: Id,
    position: Point,
    sprite_sheet: Rc<SpriteSheet>,
    animation: Sequence,
}

impl TurboFish {
    const DEFAULT_VELOCITY: f32 = 100.0 / 1000.0;

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

        let frames = (0..TURBO_FISH_ANIMATION_FRAMES).map(|i| {
            let cell_name = format!("turbo_fish_yellow_{}.png", i);
            let duration = 100.0;
            Frame::new(cell_name, duration)
        });
        Sequence::new(frames.collect())
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn bounding_box(&self) -> Rect {
        Rect::new(self.position, Self::get_shape(&self.sprite_sheet))
    }

    pub fn update(&mut self, delta_ms: f32) -> Option<GameCommand> {
        self.animation.update(delta_ms);
        self.position.x += (Self::DEFAULT_VELOCITY * delta_ms).round() as i16;

        None
    }

    pub fn draw(&self) -> Option<DrawCommand> {
        let cell = self
            .sprite_sheet
            .cell(self.animation.current_frame_cell_name())
            .expect("cell not found")
            .clone();
        let sprite_sheet = self.sprite_sheet.clone();
        let position = self.position;

        Some(DrawCommand(
            layers::ENEMY,
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
            GameCharacter::Missile(_) => Some(GameCommand::DestroyCharacter(self.id().clone())),
            _ => None,
        }
    }
}

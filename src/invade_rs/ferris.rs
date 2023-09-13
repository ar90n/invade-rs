use std::rc::Rc;

use crate::engine::geometry::{Point, Rect, Shape};
use crate::engine::sequence::{Frame, Sequence};
use crate::engine::sprite::{Cell, SpriteSheet};
use crate::engine::DrawCommand;

use super::beam::{Beam, BeamColor};
use super::character::{layers, GameCharacter, GameCommand, Id};
use super::wall::WallType;

#[derive(Clone, Copy)]
enum FerrisState {
    Idle,
    MovingLeft,
    MovingRight,
    TurnLeft(i16),
    TurnRight(i16),
}

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
    state: FerrisState,
}

impl Ferris {
    const SPAWN_BEAM_RATIO: f32 = 0.00010;
    const DEFAULT_VELOCITY: f32 = 80.0 / 1000.0;

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
            state: FerrisState::MovingLeft,
        }
    }

    fn get_beam_spawn_point(&self) -> Point {
        let cell = self.get_current_frame_cell().expect("cell not found");
        let ferris_shape = cell.shape();
        let beam_shape = Beam::get_shape(&self.sprite_sheet);

        Point {
            x: self.position.x + ferris_shape.width / 2 - beam_shape.width / 2,
            y: self.position.y + ferris_shape.height,
        }
    }

    fn get_current_frame_cell(&self) -> Option<&Cell> {
        self.sprite_sheet
            .cell(self.animation.current_frame_cell_name())
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

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn bounding_box(&self) -> Rect {
        let cell = self.get_current_frame_cell().expect("cell not found");
        let shape = cell.shape();
        let position = self.position.clone();

        Rect::new(position, shape)
    }

    pub fn update(&mut self, delta_ms: f32) -> Option<GameCommand> {
        self.animation.update(delta_ms);
        self.position.x += (self.get_velocity_x() * delta_ms).round() as i16;
        self.position.y += (self.get_velocity_y() * delta_ms).round() as i16;

        match self.state {
            FerrisState::TurnLeft(ahead_position_y) if ahead_position_y < self.position.y => {
                self.position.y = ahead_position_y;
                self.move_left();
            }
            FerrisState::TurnRight(ahead_position_y) if ahead_position_y < self.position.y => {
                self.position.y = ahead_position_y;
                self.move_right();
            }
            _ => {}
        }

        if rand::random::<f32>() < Self::SPAWN_BEAM_RATIO {
            let beam = Beam::new(
                self.sprite_sheet.clone(),
                self.get_beam_spawn_point(),
                self.color.into(),
            );
            return Some(GameCommand::SpawnCharacter(beam.into()));
        }

        None
    }

    pub fn draw(&self) -> Option<DrawCommand> {
        let cell = self
            .get_current_frame_cell()
            .expect("cell not found")
            .clone();
        let sprite_sheet = self.sprite_sheet.clone();
        let position = self.position.clone();

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
            GameCharacter::Missile(_) | GameCharacter::Ferris(_) => {
                Some(GameCommand::DestroyCharacter(self.id().clone()))
            }
            GameCharacter::Wall(wall) => match (wall.wall_type(), self.state) {
                (WallType::Left, FerrisState::MovingLeft) => Some(GameCommand::TurnFerris),
                (WallType::Right, FerrisState::MovingRight) => Some(GameCommand::TurnFerris),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn move_left(&mut self) {
        self.state = FerrisState::MovingLeft;
    }

    pub fn move_right(&mut self) {
        self.state = FerrisState::MovingRight;
    }

    pub fn stop(&mut self) {
        self.state = FerrisState::Idle;
    }

    pub fn start(&mut self) {
        self.state = FerrisState::MovingRight;
    }

    pub fn turn(&mut self) {
        let ahead_position_y = self.position.y + Self::get_shape(&self.sprite_sheet).height;
        self.state = match self.state {
            FerrisState::MovingLeft => FerrisState::TurnRight(ahead_position_y),
            FerrisState::MovingRight => FerrisState::TurnLeft(ahead_position_y),
            _ => self.state,
        };
    }

    fn get_velocity_x(&self) -> f32 {
        match self.state {
            FerrisState::MovingLeft => -Self::DEFAULT_VELOCITY,
            FerrisState::MovingRight => Self::DEFAULT_VELOCITY,
            _ => 0.0,
        }
    }

    fn get_velocity_y(&self) -> f32 {
        match self.state {
            FerrisState::TurnLeft(_) | FerrisState::TurnRight(_) => Self::DEFAULT_VELOCITY,
            _ => 0.0,
        }
    }
}

use std::rc::Rc;

use crate::engine::geometry::{Point, Rect, Shape};
use crate::engine::sprite::{Cell, SpriteSheet};
use crate::engine::{DrawCommand, Game};

use super::character::{Character, GameCommand, Id};
use super::missile::Missile;

#[derive(Clone)]
pub struct Ship {
    id: Id,
    position: Point,
    sprite_sheet: Rc<SpriteSheet>,
    cell: Cell,
    velocity: i16,
    need_shot: bool,
}

impl Ship {
    const VELOCITY: i16 = 1;

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
            id: Id::new(),
            position,
            sprite_sheet,
            cell,
            velocity: 0,
            need_shot: false,
        }
    }

    pub fn move_left(&mut self) {
        self.velocity = -Self::VELOCITY;
    }

    pub fn move_right(&mut self) {
        self.velocity = Self::VELOCITY;
    }

    pub fn stop(&mut self) {
        self.velocity = 0;
    }

    pub fn shot(&mut self) {
        self.need_shot = true;
    }

    fn get_missile_spawn_point(&self) -> Point {
        let ship_shape = self.cell.shape();
        let missile_shape = Missile::get_shape(&self.sprite_sheet);

        Point {
            x: self.position.x + ship_shape.width / 2 - missile_shape.width / 2,
            y: self.position.y,
        }
    }
}

impl Character for Ship {
    fn id(&self) -> &Id {
        &self.id
    }
    fn bounding_box(&self) -> Rect {
        Rect::new(self.position.clone(), self.cell.shape())
    }

    fn update(&mut self, delta: f32) -> Option<GameCommand> {
        self.position.x += self.velocity;

        if self.need_shot {
            self.need_shot = false;
            let missile = Missile::new(self.sprite_sheet.clone(), self.get_missile_spawn_point());
            Some(GameCommand::SpawnCharacter(Box::new(missile)))
        } else {
            None
        }
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

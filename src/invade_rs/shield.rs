use std::cell::RefCell;
use std::rc::Rc;

use crate::engine::geometry::{Point, Rect, Shape};
use crate::engine::sequence::{Frame, Sequence};
use crate::engine::sprite::{Cell, SpriteSheet};
use crate::engine::{Character, DrawCommand};

#[derive(Clone, Copy)]
enum ShieldType {
    Left,
    Middle,
    Right,
}

#[derive(Clone)]
pub struct ShieldElement {
    position: Point,
    sprite_sheet: Rc<SpriteSheet>,
    cell: Cell,
}

impl ShieldElement {
    pub fn get_shape(sprite_sheet: &Rc<SpriteSheet>) -> Shape {
        let cell = sprite_sheet
            .cell("shield_red_0.png")
            .expect("cell not found");
        cell.shape()
    }

    fn new(sprite_sheet: Rc<SpriteSheet>, position: Point, shield_type: ShieldType) -> Self {
        let cell_name = match shield_type {
            ShieldType::Left => "shield_red_4.png",
            ShieldType::Middle => "shield_red_0.png",
            ShieldType::Right => "shield_red_5.png",
        };
        let cell = sprite_sheet
            .cell(cell_name)
            .expect("cell not found")
            .clone();

        Self {
            position,
            sprite_sheet,
            cell,
        }
    }
}

impl Character for ShieldElement {
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

pub fn create_shield(
    sprite_sheet: Rc<SpriteSheet>,
    position: &Point,
) -> Vec<Rc<RefCell<Box<dyn Character>>>> {
    const SHIELD_COLS: i16 = 4;
    const SHIELD_ROWS: i16 = 3;

    let shield_shape = ShieldElement::get_shape(&sprite_sheet);

    let mut elements = vec![];
    for row in 0..SHIELD_ROWS {
        let y = row * shield_shape.height;
        for col in 0..SHIELD_COLS {
            let x = col * shield_shape.width;
            let position = Point {
                x: position.x + x,
                y: position.y + y,
            };
            let shield_type = match (row, col) {
                (0, 0) => ShieldType::Left,
                (0, x) if x == (SHIELD_COLS - 1) => ShieldType::Right,
                _ => ShieldType::Middle,
            };
            elements.push(Rc::new(RefCell::new(Box::new(ShieldElement::new(
                sprite_sheet.clone(),
                position,
                shield_type,
            )) as Box<dyn Character>)));
        }
    }

    elements
}

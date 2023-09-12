use uuid::Uuid;

use super::{
    beam::Beam, ferris::Ferris, missile::Missile, shield::ShieldElement, ship::Ship,
    turbo_fish::TurboFish, wall::Wall
};
use crate::engine::geometry::Rect;
use crate::engine::DrawCommand;

#[derive(Clone, Debug, PartialEq)]
pub struct Id(String);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4().to_string())
    }
}

pub enum GameCommand {
    SpawnCharacter(GameCharacter),
    DestroyCharacter(Id),
    DestroyPlayer,
    TurnFerris,
}

pub enum GameCharacter {
    Ferris(Ferris),
    Ship(Ship),
    TurboFish(TurboFish),
    Missile(Missile),
    Beam(Beam),
    ShieldElement(ShieldElement),
    Wall(Wall),
}

impl GameCharacter {
    pub fn id(&self) -> &Id {
        match self {
            Self::Ferris(character) => character.id(),
            Self::Ship(character) => character.id(),
            Self::TurboFish(character) => character.id(),
            Self::Missile(character) => character.id(),
            Self::Beam(character) => character.id(),
            Self::ShieldElement(character) => character.id(),
            Self::Wall(character) => character.id(),
        }
    }

    pub fn bounding_box(&self) -> Rect {
        match self {
            Self::Ferris(character) => character.bounding_box(),
            Self::Ship(character) => character.bounding_box(),
            Self::TurboFish(character) => character.bounding_box(),
            Self::Missile(character) => character.bounding_box(),
            Self::Beam(character) => character.bounding_box(),
            Self::ShieldElement(character) => character.bounding_box(),
            Self::Wall(character) => character.bounding_box(),
        }
    }

    pub fn update(&mut self, delta: f32) -> Option<GameCommand> {
        match self {
            Self::Ferris(character) => character.update(delta),
            Self::Ship(character) => character.update(delta),
            Self::TurboFish(character) => character.update(delta),
            Self::Missile(character) => character.update(delta),
            Self::Beam(character) => character.update(delta),
            Self::ShieldElement(character) => character.update(delta),
            Self::Wall(character) => character.update(delta),
        }
    }

    pub fn on_exit_screen(&mut self) -> Option<GameCommand> {
        match self {
            Self::Ferris(character) => character.on_exit_screen(),
            Self::Ship(character) => character.on_exit_screen(),
            Self::TurboFish(character) => character.on_exit_screen(),
            Self::Missile(character) => character.on_exit_screen(),
            Self::Beam(character) => character.on_exit_screen(),
            Self::ShieldElement(character) => character.on_exit_screen(),
            Self::Wall(character) => character.on_exit_screen(),
        }
    }

    pub fn on_collide(&self, other: &Self) -> Option<GameCommand> {
        match self {
            Self::Ferris(character) => character.on_collide(other),
            Self::Ship(character) => character.on_collide(other),
            Self::TurboFish(character) => character.on_collide(other),
            Self::Missile(character) => character.on_collide(other),
            Self::Beam(character) => character.on_collide(other),
            Self::ShieldElement(character) => character.on_collide(other),
            Self::Wall(character) => character.on_collide(other),
        }
    }

    pub fn draw(&self) -> Option<DrawCommand> {
        match self {
            Self::Ferris(character) => character.draw(),
            Self::Ship(character) => character.draw(),
            Self::TurboFish(character) => character.draw(),
            Self::Missile(character) => character.draw(),
            Self::Beam(character) => character.draw(),
            Self::ShieldElement(character) => character.draw(),
            Self::Wall(character) => character.draw(),
        }
    }
}

impl From<Ferris> for GameCharacter {
    fn from(character: Ferris) -> Self {
        Self::Ferris(character)
    }
}

impl From<Ship> for GameCharacter {
    fn from(character: Ship) -> Self {
        Self::Ship(character)
    }
}

impl From<TurboFish> for GameCharacter {
    fn from(character: TurboFish) -> Self {
        Self::TurboFish(character)
    }
}

impl From<Missile> for GameCharacter {
    fn from(character: Missile) -> Self {
        Self::Missile(character)
    }
}

impl From<Beam> for GameCharacter {
    fn from(character: Beam) -> Self {
        Self::Beam(character)
    }
}

impl From<ShieldElement> for GameCharacter {
    fn from(character: ShieldElement) -> Self {
        Self::ShieldElement(character)
    }
}

impl From<Wall> for GameCharacter {
    fn from(character: Wall) -> Self {
        Self::Wall(character)
    }
}

pub mod layers {
    pub const BACKGROUND: u8 = 0;
    pub const SHIELD: u8 = 1;
    pub const BEAM: u8 = 2;
    pub const MISSILE: u8 = 3;
    pub const SHIP: u8 = 4;
    pub const ENEMY: u8 = 5;
}

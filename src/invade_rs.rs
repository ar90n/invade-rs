use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;

use crate::engine::event::Event;
use crate::engine::geometry::{Point, Rect};
use crate::engine::renderer::Renderer;
use crate::engine::sprite::SpriteSheet;
use crate::engine::{browser, event, sprite};
use crate::engine::{DrawCommand, Game};

use self::character::{GameCharacter, GameCommand, Id};
use self::ferris::{Ferris, FerrisColor};
use self::fsm::{State, StateMachine, StateMachineRunner};
use self::shield::ShieldElement;
use self::ship::Ship;

mod beam;
mod character;
mod ferris;
mod fsm;
mod missile;
mod shield;
mod ship;
mod turbo_fish;
mod wall;

const SCREEN_RECT: Rect = Rect::new_from_x_y_w_h(0, 0, 600, 600);

#[derive(Clone)]
struct Created {}

impl State<Event, GameStateMachine> for Created {
    fn update(&self, delta: f32, _events: &[Event]) -> GameStateMachine {
        GameStateMachine::Created(self.clone())
    }

    fn on_enter(&mut self) -> Result<()> {
        browser::log("enter Created");
        Ok(())
    }

    fn on_exit(&mut self) -> Result<()> {
        browser::log("exit Created");
        Ok(())
    }
}

impl Into<GameStateMachine> for Created {
    fn into(self) -> GameStateMachine {
        GameStateMachine::Created(self)
    }
}

#[derive(Clone)]
struct OutGame {
    sprite_sheet: Rc<sprite::SpriteSheet>,
    characters: Vec<Rc<RefCell<GameCharacter>>>,
}

impl OutGame {
    fn new(
        sprite_sheet: Rc<sprite::SpriteSheet>,
        characters: Vec<Rc<RefCell<GameCharacter>>>,
    ) -> Self {
        Self {
            sprite_sheet,
            characters,
        }
    }
}

impl State<Event, GameStateMachine> for OutGame {
    fn update(&self, delta: f32, events: &[Event]) -> GameStateMachine {
        GameStateMachine::OutGame(self.clone())
    }

    fn on_enter(&mut self) -> Result<()> {
        browser::log("enter OutGame");
        Ok(())
    }

    fn on_exit(&mut self) -> Result<()> {
        browser::log("exit OutGame");
        Ok(())
    }
}

impl Into<GameStateMachine> for OutGame {
    fn into(self) -> GameStateMachine {
        GameStateMachine::OutGame(self)
    }
}

#[derive(Clone)]
struct InGame {
    sprite_sheet: Rc<sprite::SpriteSheet>,
    characters: Vec<Rc<RefCell<GameCharacter>>>,
    player: Rc<RefCell<Ship>>,
}

impl InGame {
    fn apply_event(&self, event: &Event) {
        match event {
            Event::KeyDown(key) => match key.as_str() {
                "ArrowRight" => self.player.borrow_mut().move_right(),
                "ArrowLeft" => self.player.borrow_mut().move_left(),
                "Space" => self.player.borrow_mut().shot(),
                _ => {}
            },
            Event::KeyUp(key) => match key.as_str() {
                "ArrowRight" => self.player.borrow_mut().stop(),
                "ArrowLeft" => self.player.borrow_mut().stop(),
                _ => {}
            },
        }
    }

    fn update_game(&self, delta: f32) -> Vec<GameCommand> {
        const TURBO_FISH_APPEAR_PROBABILITY: f32 = 0.001;

        let mut commands = vec![];
        for c in self.characters.iter() {
            let mut c = c.borrow_mut();

            let cur_visible = c.bounding_box().intersects(&SCREEN_RECT);
            if let Some(command) = c.update(delta) {
                commands.push(command);
            }
            let next_visible = c.bounding_box().intersects(&SCREEN_RECT);

            if cur_visible && !next_visible {
                if let Some(command) = c.on_exit_screen() {
                    commands.push(command);
                }
            }
        }

        for c in self.characters.iter() {
            let c = c.borrow();
            for other in self.characters.iter() {
                let other = other.borrow();
                if c.id() == other.id() {
                    continue;
                }

                if c.bounding_box().intersects(&other.bounding_box()) {
                    if let Some(command) = c.on_collide(&other) {
                        commands.push(command);
                    }
                }
            }
        }

        if let Some(command) = self.player.borrow_mut().update(delta) {
            commands.push(command);
        }

        if rand::random::<f32>() < TURBO_FISH_APPEAR_PROBABILITY {
            commands.push(self.create_spawn_turbo_fish_command(&self.sprite_sheet));
        }
        commands
    }

    fn apply_command(&mut self, command: GameCommand) {
        match command {
            GameCommand::SpawnCharacter(new_character) => {
                self.characters.push(Rc::new(RefCell::new(new_character)));
            }
            GameCommand::DestroyCharacter(id) => {
                self.characters.retain(|c| c.borrow().id() != &id);
            }
            GameCommand::TurnFerris => {
                for c in self.characters.iter() {
                    let mut c = c.borrow_mut();
                    if let GameCharacter::Ferris(ferris) = &mut *c {
                        ferris.turn();
                    }
                }
            }
            _ => {}
        }
    }

    fn new(
        sprite_sheet: Rc<sprite::SpriteSheet>,
        characters: Vec<Rc<RefCell<GameCharacter>>>,
        player: Rc<RefCell<Ship>>,
    ) -> Self {
        Self {
            sprite_sheet,
            characters,
            player,
        }
    }

    fn create_spawn_turbo_fish_command(&self, sprite_sheet: &Rc<SpriteSheet>) -> GameCommand {
        const Y_ORIGIN: i16 = 50;

        let ship_shape = turbo_fish::TurboFish::get_shape(sprite_sheet);
        let x_origin = -ship_shape.width;

        let position = Point {
            x: x_origin,
            y: Y_ORIGIN,
        };
        let turbo_fish = turbo_fish::TurboFish::new(sprite_sheet.clone(), position);
        GameCommand::SpawnCharacter(turbo_fish.into())
    }
}

impl State<Event, GameStateMachine> for InGame {
    fn update(&self, delta: f32, events: &[Event]) -> GameStateMachine {
        events.iter().for_each(|event| self.apply_event(event));

        let commands = self.update_game(delta);

        let next_state = {
            let mut next_state = self.clone();
            for c in commands.into_iter() {
                next_state.apply_command(c);
            }
            next_state
        };
        GameStateMachine::InGame(next_state)
    }

    fn on_enter(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_exit(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Into<GameStateMachine> for InGame {
    fn into(self) -> GameStateMachine {
        GameStateMachine::InGame(self)
    }
}

enum GameStateMachine {
    Created(Created),
    OutGame(OutGame),
    InGame(InGame),
}

impl GameStateMachine {
    fn draw(&self) -> Vec<DrawCommand> {
        match self {
            Self::Created(_) => vec![],
            Self::OutGame(state) => state
                .characters
                .iter()
                .map(|c| c.borrow().draw())
                .filter_map(|c| c)
                .collect(),
            Self::InGame(state) => {
                let mut draw_commands = vec![];
                draw_commands
                    .append(&mut state.characters.iter().map(|c| c.borrow().draw()).collect());
                draw_commands.push(state.player.borrow().draw());
                draw_commands.into_iter().filter_map(|c| c).collect()
            }
        }
    }
}

impl StateMachine<Event> for GameStateMachine {
    fn update(&self, delta: f32, events: &[Event]) -> Self {
        match self {
            Self::Created(state) => state.update(delta, events),
            Self::OutGame(state) => state.update(delta, events),
            Self::InGame(state) => state.update(delta, events),
        }
    }

    fn on_enter(&mut self) -> Result<()> {
        match self {
            Self::Created(state) => state.on_enter(),
            Self::OutGame(state) => state.on_enter(),
            Self::InGame(state) => state.on_enter(),
        }
    }

    fn on_exit(&mut self) -> Result<()> {
        match self {
            Self::Created(state) => state.on_exit(),
            Self::OutGame(state) => state.on_exit(),
            Self::InGame(state) => state.on_exit(),
        }
    }
}

impl Default for GameStateMachine {
    fn default() -> Self {
        Self::Created(Created {})
    }
}

impl PartialEq for GameStateMachine {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Created(_), Self::Created(_)) => true,
            (Self::OutGame(_), Self::OutGame(_)) => true,
            (Self::InGame(_), Self::InGame(_)) => true,
            _ => false,
        }
    }
}

pub struct InvadeRs {
    runner: StateMachineRunner<Event, GameStateMachine>,
}

impl InvadeRs {
    pub fn new() -> Self {
        Self {
            runner: StateMachineRunner::new(),
        }
    }

    async fn load_sprite_sheet(&mut self) -> Result<Rc<SpriteSheet>> {
        sprite::SpriteSheet::load("texture.json", "texture.png")
            .await
            .map(|sprite_sheet| Rc::new(sprite_sheet))
    }

    fn spawn_ferris_fleet(
        &self,
        sprite_sheet: &Rc<SpriteSheet>,
        screen_width: i16,
    ) -> Vec<Rc<RefCell<GameCharacter>>> {
        const FLEET_COLS: i16 = 9;
        const FLEET_ROWS: i16 = 2 * 3;
        const Y_ORIGIN: i16 = 100;
        const MARGIN: i16 = 10;

        let ferris_shape = Ferris::get_shape(sprite_sheet);
        let x_origin = (screen_width - FLEET_COLS * (ferris_shape.width + MARGIN)) / 2;
        let colors = vec![FerrisColor::Magenta, FerrisColor::Green, FerrisColor::Blue];

        let mut characters = vec![];
        for row in 0..FLEET_ROWS {
            let color = colors[(row / 2) as usize];
            let y = Y_ORIGIN + row * (ferris_shape.height + MARGIN);
            for col in 0..FLEET_COLS {
                let x = x_origin + (MARGIN / 2) + col * (ferris_shape.width + MARGIN);
                let position = Point { x, y };
                let ferris = Ferris::new(sprite_sheet.clone(), position, color);
                characters.push(Rc::new(RefCell::new(ferris.into())));
            }
        }

        characters
    }

    fn spawn_aligned_shields(
        &self,
        sprite_sheet: &Rc<SpriteSheet>,
        screen_width: i16,
    ) -> Vec<Rc<RefCell<GameCharacter>>> {
        const SHIELD_NUM: i16 = 4;
        const Y_ORIGIN: i16 = 490;

        let shield_element_shape = ShieldElement::get_shape(sprite_sheet);
        let shield_width = 4 * shield_element_shape.width;
        let margin = (screen_width - SHIELD_NUM * shield_width) / (2 * SHIELD_NUM);

        let mut characters = vec![];
        for i in 0..SHIELD_NUM {
            let x = margin + i * (shield_width + 2 * margin);
            let position = Point { x, y: Y_ORIGIN };
            shield::create_shield(sprite_sheet.clone(), &position)
                .into_iter()
                .for_each(|c| characters.push(Rc::new(RefCell::new(c.into()))));
        }

        characters
    }

    fn spawn_ship(&self, sprite_sheet: &Rc<SpriteSheet>, screen_width: i16) -> Rc<RefCell<Ship>> {
        const Y_ORIGIN: i16 = 560;

        let ship_shape = ship::Ship::get_shape(sprite_sheet);
        let x_origin = (screen_width - ship_shape.width) / 2;

        let position = Point {
            x: x_origin,
            y: Y_ORIGIN,
        };
        let ship = ship::Ship::new(sprite_sheet.clone(), position);
        Rc::new(RefCell::new(ship.into()))
    }

    fn spawn_walls(
        &self,
        screen_width: i16,
        screen_height: i16,
    ) -> Vec<Rc<RefCell<GameCharacter>>> {
        let left_wall = wall::Wall::new(
            wall::WallType::Left,
            Rect::new_from_x_y_w_h(0, 0, 1, screen_height),
        );
        let right_wall = wall::Wall::new(
            wall::WallType::Right,
            Rect::new_from_x_y_w_h(screen_width - 1, 0, 1, screen_height),
        );

        vec![
            Rc::new(RefCell::new(left_wall.into())),
            Rc::new(RefCell::new(right_wall.into())),
        ]
    }
}

impl Default for InvadeRs {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Game for InvadeRs {
    async fn initialize(&mut self) -> Result<()> {
        let sprite_sheet = self.load_sprite_sheet().await?;
        let characters = {
            let mut characters = vec![];
            characters.append(&mut self.spawn_ferris_fleet(&sprite_sheet, SCREEN_RECT.width()));
            characters.append(&mut self.spawn_aligned_shields(&sprite_sheet, SCREEN_RECT.width()));
            characters.append(&mut self.spawn_walls(SCREEN_RECT.width(), SCREEN_RECT.height()));
            characters
        };
        let player = self.spawn_ship(&sprite_sheet, SCREEN_RECT.width());

        self.runner
            //.transition(OutGame::new(sprite_sheet, characters).into())?;
            .transition(InGame::new(sprite_sheet, characters, player).into())?;
        Ok(())
    }

    fn update(&mut self, delta: f32, events: &[Event]) -> Result<()> {
        self.runner.update(delta, events)
    }

    fn draw(&self) -> Vec<DrawCommand> {
        let clear_command = DrawCommand(
            0,
            Box::new(|renderer: &dyn Renderer| {
                renderer.clear(&SCREEN_RECT);
            }),
        );

        let mut draw_commands = vec![clear_command];
        draw_commands.append(self.runner.state.draw().as_mut());
        draw_commands
    }
}

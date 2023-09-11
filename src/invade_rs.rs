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

use self::character::{Character, GameCommand, Id};
use self::ferris::{Ferris, FerrisColor};
use self::fsm::{State, StateMachine, StateMachineRunner};
use self::shield::ShieldElement;
use self::ship::Ship;
use self::turbo_fish::TurboFish;

mod beam;
mod character;
mod ferris;
mod fsm;
mod missile;
mod shield;
mod ship;
mod turbo_fish;

#[derive(Clone)]
struct Created {}

#[async_trait(?Send)]
impl State<GameStateMachine> for Created {
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
    characters: Vec<Rc<RefCell<Box<dyn Character>>>>,
}

impl OutGame {
    fn new(
        sprite_sheet: Rc<sprite::SpriteSheet>,
        characters: Vec<Rc<RefCell<Box<dyn Character>>>>,
    ) -> Self {
        Self {
            sprite_sheet,
            characters,
        }
    }
}

#[async_trait(?Send)]
impl State<GameStateMachine> for OutGame {
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
    characters: Vec<Rc<RefCell<Box<dyn Character>>>>,
    player: Rc<RefCell<Ship>>,
}

impl InGame {
    fn new(
        sprite_sheet: Rc<sprite::SpriteSheet>,
        characters: Vec<Rc<RefCell<Box<dyn Character>>>>,
        player: Rc<RefCell<Ship>>,
    ) -> Self {
        Self {
            sprite_sheet,
            characters,
            player,
        }
    }

    fn spawn_turbo_fish(&self, sprite_sheet: &Rc<SpriteSheet>) -> Rc<RefCell<Box<dyn Character>>> {
        const Y_ORIGIN: i16 = 50;

        let ship_shape = turbo_fish::TurboFish::get_shape(sprite_sheet);
        //let x_origin =  -ship_shape.width;
        let x_origin = 0;

        let position = Point {
            x: x_origin,
            y: Y_ORIGIN,
        };
        let turbo_fish = turbo_fish::TurboFish::new(sprite_sheet.clone(), position);
        Rc::new(RefCell::new(Box::new(turbo_fish) as Box<dyn Character>))
    }
}

#[async_trait(?Send)]
impl State<GameStateMachine> for InGame {
    fn update(&self, delta: f32, events: &[Event]) -> GameStateMachine {
        const TURBO_FISH_APPEAR_PROBABILITY: f32 = 0.001;


        let mut next_state = self.clone();

        browser::log(&format!("InGame update: {} {}", delta, next_state.characters.len()));

        events.iter().for_each(|event| match event {
            Event::KeyDown(key) => match key.as_str() {
                "ArrowRight" => next_state.player.borrow_mut().move_right(),
                "ArrowLeft" => next_state.player.borrow_mut().move_left(),
                "Space" => next_state.player.borrow_mut().shot(),
                _ => {}
            },
            Event::KeyUp(key) => match key.as_str() {
                "ArrowRight" => next_state.player.borrow_mut().stop(),
                "ArrowLeft" => next_state.player.borrow_mut().stop(),
                _ => {}
            },
        });

        let mut cur_visible = vec![];
        let mut next_visible = vec![];
        let mut commands = vec![];
        for c in next_state.characters.iter_mut() {
            let mut c= c.borrow_mut();

            cur_visible.push(c.bounding_box().intersects(&Rect::new_from_x_y_w_h(0, 0, 600, 600)));
            let command = c.update(delta);
            if let Some(command) = command {
                commands.push(command);
            }
            next_visible.push(c.bounding_box().intersects(&Rect::new_from_x_y_w_h(0, 0, 600, 600)));
        }

        for ((c, cur_visible), next_visible) in next_state
            .characters
            .iter_mut()
            .zip(cur_visible.into_iter())
            .zip(next_visible.into_iter())
        {
            if cur_visible && !next_visible {
                let command = c.borrow_mut().on_exit_screen();
                if let Some(command) = command {
                    commands.push(command);
                }
            }
        }

        let command = next_state.player.borrow_mut().update(delta);
        if let Some(command) = command {
            commands.push(command);
        }


        if rand::random::<f32>() < TURBO_FISH_APPEAR_PROBABILITY {
            next_state
                .characters
                .push(self.spawn_turbo_fish(&self.sprite_sheet));
        }

        for c in commands.into_iter() {
            match c {
                GameCommand::SpawnCharacter(new_character) => {
                    next_state
                        .characters
                        .push(Rc::new(RefCell::new(new_character)));
                }
                GameCommand::DestroyCharacter(id) => {
                    next_state
                        .characters
                        .retain(|c| c.borrow().id() != &id);
                }
                _ => {}
            }
        }

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

#[async_trait(?Send)]
impl StateMachine for GameStateMachine {
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
    runner: StateMachineRunner<GameStateMachine>,
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

    async fn load_audio(&mut self) -> Result<()> {
        todo!()
        //        let audio = Audio::new()?;
        //        let sound = audio.load_sound("SFX_Jump_23.mp3").await?;
        //        let background_music = audio.load_sound("background_song.mp3").await?;
    }

    fn spawn_ferris_fleet(
        &self,
        sprite_sheet: &Rc<SpriteSheet>,
        screen_width: i16,
    ) -> Vec<Rc<RefCell<Box<dyn Character>>>> {
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
                characters.push(Rc::new(
                    RefCell::new(Box::new(ferris) as Box<dyn Character>),
                ));
            }
        }

        characters
    }

    fn spawn_aligned_shields(
        &self,
        sprite_sheet: &Rc<SpriteSheet>,
        screen_width: i16,
    ) -> Vec<Rc<RefCell<Box<dyn Character>>>> {
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
                .for_each(|c| {
                    characters.push(Rc::new(RefCell::new(Box::new(c) as Box<dyn Character>)))
                });
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
            characters.append(&mut self.spawn_ferris_fleet(&sprite_sheet, 600));
            characters.append(&mut self.spawn_aligned_shields(&sprite_sheet, 600));
            characters
        };
        let player = self.spawn_ship(&sprite_sheet, 600);

        self.runner
            //.transition(OutGame::new(sprite_sheet, characters).into())?;
            .transition(InGame::new(sprite_sheet, characters, player).into())?;

        //        audio.play_looping_sound(&background_music)?;
        //        let rhb = RedHatBoy::new(
        //            sheet.into_serde::<Sheet>()?,
        //            engine::load_image("rhb.png").await?,
        //            audio,
        //            sound,
        //        );

        //        let background_width = background.width() as i16;
        //        let starting_obstacles = stone_and_platform(stone.clone(), sprite_sheet.clone(), 0);
        //        let timeline = rightmost(&starting_obstacles);
        //        let machine = WalkTheDogStateMachine::Ready(WalkTheDogState {
        //            _state: Read,
        //            walk: Walk {
        //                boy: rhb,
        //                backgrounds: [
        //                    Image::new(background.clone(), Point { x: 0, y: 0 }),
        //                    Image::new(
        //                        background,
        //                        Point {
        //                            x: background_width,
        //                            y: 0,
        //                        },
        //                    ),
        //                ],
        //                obstacles: starting_obstacles,
        //                obstacle_sheet: sprite_sheet,
        //                stone,
        //                timeline,
        //            },
        //        });
        Ok(())
    }

    fn update(&mut self, delta: f32, events: &[Event]) -> Result<()> {
        self.runner.update(delta, events)
    }

    fn draw(&self) -> Vec<DrawCommand> {
        let clear_command = DrawCommand(
            0,
            Box::new(|renderer: &dyn Renderer| {
                renderer.clear(&Rect::new_from_x_y_w_h(0, 0, 600, 600));
            }),
        );

        let mut draw_commands = vec![clear_command];
        draw_commands.append(self.runner.state.draw().as_mut());
        draw_commands
    }
}

use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;

use crate::engine::geometry::{Point, Rect};
use crate::engine::{browser, event, renderer, sprite};
use crate::engine::{Character, Drawable, Game};

use self::ferris::{Ferris, FerrisColor};

mod ferris;


pub struct InvadeRs {
    sprite_sheet: Option<Rc<sprite::SpriteSheet>>,
    ferris: Option<Ferris>,
}

impl InvadeRs {
    pub fn new() -> Self {
        Self {
            sprite_sheet: None,
            ferris: None,
        }
    }

    async fn load_sprite_sheet(&mut self) -> Result<()> {
        self.sprite_sheet = sprite::SpriteSheet::load("texture.json", "texture.png")
            .await
            .map(|sprite_sheet| Rc::new(sprite_sheet))
            .ok();

        Ok(())
    }

    async fn load_audio(&mut self) -> Result<()> {
        todo!()
        //        let audio = Audio::new()?;
        //        let sound = audio.load_sound("SFX_Jump_23.mp3").await?;
        //        let background_music = audio.load_sound("background_song.mp3").await?;
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
        self.load_sprite_sheet().await?;

        self.ferris = Some(Ferris::new(
            self.sprite_sheet.clone().unwrap(),
            Point { x: 0, y: 0 },
            FerrisColor::Blue,
        ));

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

    fn apply_event(&mut self, event: event::Event) {}

    fn update(&mut self, delta: f32) {
        if let Some(ferris) = self.ferris.as_mut() {
            browser::log(format!("delta: {}", delta).as_str());
            ferris.update(delta);
        }
    }

    fn draw(&self, renderer: &impl renderer::Renderer) {
        renderer.clear(&Rect::new_from_x_y_w_h(0, 0, 600, 600));

        if let Some(ferris) = self.ferris.as_ref() {
            ferris.draw(renderer);
        }
    }
}

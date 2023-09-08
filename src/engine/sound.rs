use anyhow::{anyhow, Result};
use js_sys::ArrayBuffer;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, AudioDestinationNode, AudioNode};

use super::browser;

pub fn create_audio_context() -> Result<AudioContext> {
    AudioContext::new().map_err(|err| anyhow!("Could not create audio context: {:#?}", err))
}

fn create_buffer_source(ctx: &AudioContext) -> Result<AudioBufferSourceNode> {
    ctx.create_buffer_source()
        .map_err(|err| anyhow!("Error creating buffer source {:#?}", err))
}

fn connect_with_audio_node(
    buffer_source: &AudioBufferSourceNode,
    destination: &AudioDestinationNode,
) -> Result<AudioNode> {
    buffer_source
        .connect_with_audio_node(&destination)
        .map_err(|err| anyhow!("Error connecting audio source to destination {:#?}", err))
}

pub async fn decode_audio_data(
    ctx: &AudioContext,
    array_buffer: &ArrayBuffer,
) -> Result<AudioBuffer> {
    JsFuture::from(
        ctx.decode_audio_data(&array_buffer)
            .map_err(|err| anyhow!("Could not decode audio from array buffer {:#?}", err))?,
    )
    .await
    .map_err(|err| anyhow!("Could not convert promise to future {:#?}", err))?
    .dyn_into()
    .map_err(|err| anyhow!("Could not convert promise to ARrayBUffer {:#?}", err))
}

fn create_track_source(ctx: &AudioContext, buffer: &AudioBuffer) -> Result<AudioBufferSourceNode> {
    let track_source = create_buffer_source(ctx)?;
    track_source.set_buffer(Some(&buffer));
    connect_with_audio_node(&track_source, &ctx.destination())?;
    Ok(track_source)
}

pub enum LOOPING {
    NO,
    YES,
}

pub fn play_sound(ctx: &AudioContext, buffer: &AudioBuffer, looping: LOOPING) -> Result<()> {
    let track_source = create_track_source(ctx, buffer)?;
    if matches!(looping, LOOPING::YES) {
        track_source.set_loop(true);
    }

    track_source
        .start()
        .map_err(|err| anyhow!("Could not start sound! {:#?}", err))
}

#[derive(Clone)]
pub struct Audio {
    context: AudioContext,
}

impl Audio {
    pub fn new() -> Result<Self> {
        Ok(Audio {
            context: create_audio_context()?,
        })
    }

    pub async fn load_sound(&self, filename: &str) -> Result<Sound> {
        let array_buffer = browser::fetch_array_buffer(filename).await?;
        let audio_buffer = decode_audio_data(&self.context, &array_buffer).await?;
        Ok(Sound {
            buffer: audio_buffer,
        })
    }

    pub fn play_sound(&self, sound: &Sound) -> Result<()> {
        play_sound(&self.context, &sound.buffer, LOOPING::NO)
    }

    pub fn play_looping_sound(&self, sound: &Sound) -> Result<()> {
        play_sound(&self.context, &sound.buffer, LOOPING::YES)
    }
}

#[derive(Clone)]
pub struct Sound {
    buffer: AudioBuffer,
}

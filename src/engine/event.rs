use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use wasm_bindgen::prelude::*;

use super::browser;

enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}

fn prepare_key_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);

    let onkeydown = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        keydown_sender
            .borrow_mut()
            .start_send(KeyPress::KeyDown(keycode))
            .expect("Failed to start KeyDown send");
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    let onkeyup = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        keyup_sender
            .borrow_mut()
            .start_send(KeyPress::KeyUp(keycode))
            .expect("Failed to start KeyPPresssresss send");
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    browser::canvas()
        .unwrap()
        .set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    browser::canvas()
        .unwrap()
        .set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    onkeydown.forget();
    onkeyup.forget();

    Ok(keyevent_receiver)
}

pub enum Event {
    KeyUp(String),
    KeyDown(String),
}

impl From<KeyPress> for Event {
    fn from(value: KeyPress) -> Self {
        match value {
            KeyPress::KeyUp(evt) => Event::KeyUp(evt.code()),
            KeyPress::KeyDown(evt) => Event::KeyDown(evt.code()),
        }
    }
}

pub trait EventSource {
    fn try_next(&mut self) -> Option<Event>;
}

pub struct BrowserEventSource {
    keyevent_receiver: UnboundedReceiver<KeyPress>,
}

impl BrowserEventSource {
    pub fn new() -> Result<Self> {
        let keyevent_receiver = prepare_key_input()?;
        Ok(Self { keyevent_receiver })
    }
}

impl EventSource for BrowserEventSource {
    fn try_next(&mut self) -> Option<Event> {
        match self.keyevent_receiver.try_next() {
            Ok(Some(evt)) => Some(evt.into()),
            _ => None,
        }
    }
}

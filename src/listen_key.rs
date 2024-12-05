use crossterm::event::{read, Event, KeyEvent, KeyEventKind,KeyCode};
use std::{io::Result, sync::{Arc, Mutex}};
use crate::game::{Direction,Game};
pub fn listen_keyboard(d:Arc<Mutex<Direction>>) -> Result<()> {
    loop {
        match read()? {
            Event::Key(e) => {
                if e.kind == KeyEventKind::Press {
                    match e.code {
                        KeyCode::Up => {
                            *d.lock().unwrap() = Direction::Up;
                            dbg!("Player input Up!");
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }
}

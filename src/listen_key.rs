use crate::game::{Direction, Game};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, read};
use std::{
    io::Result,
    sync::{Arc, Mutex},
};
pub fn listen_keyboard(d: Arc<Mutex<Direction>>) -> Result<()> {
    loop {
        match read()? {
            Event::Key(e) => {
                if e.kind == KeyEventKind::Press {
                    match e.code {
                        KeyCode::Up => *d.lock().unwrap() = Direction::Up,
                        KeyCode::Down => *d.lock().unwrap() = Direction::Down,
                        KeyCode::Left => *d.lock().unwrap() = Direction::Left,
                        KeyCode::Right => *d.lock().unwrap() = Direction::Right,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

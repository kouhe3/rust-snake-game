use input::event::keyboard::{KeyState, KeyboardEventTrait, KeyboardKeyEvent};
use input::event::{Event, KeyboardEvent};
use input::{Libinput, LibinputInterface};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::{
    fs::{File, OpenOptions},
    os::fd::OwnedFd,
};

use libc::{O_RDONLY, O_RDWR, O_WRONLY};

use tanchishe::{Direction, DIRECTION};

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        let _ = File::from(fd);
    }
}

pub fn create_hook() {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            match event {
                Event::Keyboard(keyboard_event) => match keyboard_event {
                    KeyboardEvent::Key(key_event) => {
                        match key_event.key_state() {
                            KeyState::Pressed => {
                                println!("{}", key_event.key());
                                match key_event.key() {
                                    103 => *DIRECTION.lock().unwrap() = Direction::Up,
                                    108 => *DIRECTION.lock().unwrap() = Direction::Down,
                                    105 => *DIRECTION.lock().unwrap() = Direction::Left,
                                    106 => *DIRECTION.lock().unwrap() = Direction::Right,
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                },
                _ => {},
            }
        }
    }
}


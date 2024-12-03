use std::sync::{Arc, LazyLock, Mutex};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub static DIRECTION: LazyLock<Arc<Mutex<Direction>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Direction::Up)));

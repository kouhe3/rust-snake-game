use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
    io::stdout,
};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use tanchishe::game::{Game, Direction};
use tanchishe::render::*;
use tanchishe::listen_key::listen_keyboard;

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    
    let mut game = Game::new(50, 20);
    let player_input_clone = Arc::clone(&game.player_input);
    let keyboard_thread = thread::spawn(|| listen_keyboard(player_input_clone));

    loop {
        game.step();
        game_display(&game)?;
        thread::sleep(Duration::from_millis(500));
    }
    /* disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(()) */
}

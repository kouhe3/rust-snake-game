use std::{
    sync::Arc,
    thread,
    time::Duration,
    io::stdout,
};
use crossterm::{
    event::{read, poll, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use tanchishe::game::{Game, Direction};
use tanchishe::render::*;

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    
    let mut game = Game::new(50, 20);
    let d = Arc::clone(&game.input);

    'main_loop: loop {
        while poll(Duration::from_secs(0))? {
            if let Event::Key(e) = read()? {
                if e.kind == KeyEventKind::Press {
                    match e.code {
                        KeyCode::Up => d.lock().unwrap().set(Direction::Up),
                        KeyCode::Down => d.lock().unwrap().set(Direction::Down),
                        KeyCode::Left => d.lock().unwrap().set(Direction::Left),
                        KeyCode::Right => d.lock().unwrap().set(Direction::Right),
                        KeyCode::Char('q') => break 'main_loop,
                        _ => {}
                    }
                }
            }
        }
        game.step();
        game_display(&game)?;
        thread::sleep(Duration::from_millis(16));
    };
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

use crate::game::Game;
use std::io::{self, stdout, Write};
use crossterm::{cursor::{MoveTo, MoveLeft, MoveDown}, execute, terminal::{Clear, ClearType}};

fn horizontal_line(length: u16) -> io::Result<()> {
    for _ in 0..length {
        print!("-");
    };
    Ok(())
}

fn vertical_line(length: u16) -> io::Result<()> {
    for _ in 0..length {
        print!("|");
        execute!(stdout(), MoveLeft(1), MoveDown(1))?;
    };
    Ok(())
}

pub fn game_display(game: &Game) -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    // corner
    execute!(stdout(), MoveTo(0, 0))?;
    print!("+");
    execute!(stdout(), MoveTo(game.stage.x + 1, 0))?;
    print!("+");
    execute!(stdout(), MoveTo(0, game.stage.y + 1))?;
    print!("+");
    execute!(stdout(), MoveTo(game.stage.x, game.stage.y + 1))?;
    print!("+");
    // horizontal line
    execute!(stdout(), MoveTo(1, 0))?;
    horizontal_line(game.stage.x)?;
    execute!(stdout(), MoveTo(1, game.stage.y + 1))?;
    horizontal_line(game.stage.x)?;
    io::stdout().flush()?;
    // vertical line
    execute!(stdout(), MoveTo(0, 1))?;
    vertical_line(game.stage.y)?;
    execute!(stdout(), MoveTo(game.stage.x + 1, 1))?;
    vertical_line(game.stage.y)?;
    let g = game.snake.lock().unwrap();
    let b = g.body.lock().unwrap();
    for body in b.iter() {
        execute!(stdout(), MoveTo(body.x, body.y))?;
        print!("#");
    }
    execute!(stdout(), MoveTo(game.food.x, game.food.y))?;
    print!("*");
    if game.game_over {
        execute!(stdout(), MoveTo(0, game.stage.y + 3))?;
        println!("You dead!");
    }
    Ok(())
}

use crate::game::Game;
use std::io::{self, stdout, Write};
use crossterm::{cursor::{MoveTo, MoveLeft, MoveDown}, execute, terminal::{Clear, ClearType}};

const HEIGHT: u16 = 20;
const WIDTH: u16 = 50;

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
    execute!(stdout(), MoveTo(WIDTH + 1, 0))?;
    print!("+");
    execute!(stdout(), MoveTo(0, HEIGHT + 1))?;
    print!("+");
    execute!(stdout(), MoveTo(WIDTH + 1, HEIGHT + 1))?;
    print!("+");
    // horizontal line
    execute!(stdout(), MoveTo(1, 0))?;
    horizontal_line(WIDTH)?;
    execute!(stdout(), MoveTo(1, HEIGHT + 1))?;
    horizontal_line(WIDTH)?;
    io::stdout().flush()?;
    // vertical line
    execute!(stdout(), MoveTo(0, 1))?;
    vertical_line(HEIGHT)?;
    execute!(stdout(), MoveTo(WIDTH + 1, 1))?;
    vertical_line(HEIGHT)?;
    let g = game.snake.lock().unwrap();
    let b = g.body.lock().unwrap();
    for body in b.iter() {
        execute!(stdout(), MoveTo(body.x, body.y))?;
        print!("#");
    }
    execute!(stdout(), MoveTo(game.food.x, game.food.y))?;
    print!("*");
    if game.game_over {
        execute!(stdout(), MoveTo(0, HEIGHT + 3))?;
        println!("You dead!");
    }
    Ok(())
}

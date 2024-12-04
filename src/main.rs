use core::panic;
use rand::Rng;
use std::mem;
use std::sync::{LazyLock, Mutex};
use std::time::Duration;
use std::{collections::VecDeque, sync::Arc, thread};
use windows::Win32::Foundation::LRESULT;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageA, GetMessageA, HC_ACTION, PostQuitMessage, TranslateMessage,
    WM_KEYDOWN,
};
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::WindowsAndMessaging::{KBDLLHOOKSTRUCT, SetWindowsHookExA, WH_KEYBOARD_LL},
};
struct Stage {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Body {
    x: u32,
    y: u32,
}

struct Food {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}

struct Snake {
    body: Mutex<VecDeque<Body>>,
    direction: Mutex<Direction>,
}

struct Game {
    snake: Mutex<Snake>,
    player_input: Arc<Mutex<Direction>>,
    food: Food,
    score: u32,
    stage: Stage,
    game_over: bool,
}

impl Game {
    fn new(x: u32, y: u32) -> Self {
        let mut rng = rand::thread_rng();
        let snake = Snake::new(Stage { x: x, y: y });
        let food = Food {
            x: rng.gen_range(0..x),
            y: rng.gen_range(0..y),
        };
        let player_input = Arc::new(Mutex::new(Direction::Right));
        Game {
            snake: Mutex::new(snake),
            player_input,
            food,
            score: 0,
            stage: Stage { x: x, y: y },
            game_over: false,
        }
    }

    fn step(&mut self){
        let mut snake = self.snake.lock().unwrap();
        snake.add_head(Direction::Right);
        snake.del_tail();
        
    }
}

impl Snake {
    fn new(stage: Stage) -> Self {
        let mut body = VecDeque::new();
        let mut rng = rand::thread_rng();
        let start_x = rng.gen_range(0..stage.x);
        let start_y = rng.gen_range(0..stage.y);
        for _ in 0..2 {
            body.push_back(Body {
                x: start_x,
                y: start_y,
            });
        }
        Snake {
            body: Mutex::new(body),
            direction: Mutex::new(Direction::Right),
        }
    }

    fn add_head(&mut self, intput_direction: Direction) {
        let mut snake = self.body.lock().unwrap();
        let mut snake_direction = self.direction.lock().unwrap();
        //check if player input oppside snake self,
        //if oppside then just ignore player input do not change current direction
        if snake_direction.reverse() != intput_direction {
            *snake_direction = intput_direction;
        }
        let new_head = match *snake_direction {
            Direction::Up => Body {
                x: snake[0].x,
                y: snake[0].y - 1,
            },
            Direction::Down => Body {
                x: snake[0].x,
                y: snake[0].y + 1,
            },
            Direction::Left => Body {
                x: snake[0].x - 1,
                y: snake[0].y,
            },
            Direction::Right => Body {
                x: snake[0].x + 1,
                y: snake[0].y,
            },
        };
        snake.push_front(new_head);
        drop(snake_direction);
        drop(snake);
    }

    fn del_tail(&mut self) {
        let mut snake = self.body.lock().unwrap();
        snake.pop_back();
        drop(snake);
    }
}

fn main() {
    let game = Game::new(100, 100);

}

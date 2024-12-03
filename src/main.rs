use rand::Rng;
use std::sync::Mutex;
use std::time::Duration;
use std::{collections::VecDeque, thread};

use tanchishe::{DIRECTION, Direction};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

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

static FOOD: Mutex<Food> = Mutex::new(Food { x: 0, y: 0 });
static STAGE: Stage = Stage { x: 100, y: 100 };
static SNAKE: Mutex<VecDeque<Body>> = Mutex::new(VecDeque::new());

fn add_snake_head() {
    let mut snake = SNAKE.lock().unwrap();
    let direct = DIRECTION.lock().unwrap();
    let new_head = match *direct {
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
}

fn del_snake_tail() {
    let mut snake = SNAKE.lock().unwrap();
    let food = FOOD.lock().unwrap();
    let eaten = snake[0].x == food.x && snake[0].y == food.y;
    if !eaten {
        //del_snake_tail
        snake.pop_back();
    } else {
        new_food();
    }
}

fn new_food() {
    let mut rng = rand::thread_rng();
    let mut food = FOOD.lock().unwrap();
    food.x = rng.gen_range(0..STAGE.x);
    food.y = rng.gen_range(0..STAGE.y);
}

fn init_snake() {
    let mut rng = rand::thread_rng();
    let mut snake = SNAKE.lock().unwrap();
    let start_x: u32 = rng.gen_range(0..STAGE.x);
    let start_y: u32 = rng.gen_range(0..STAGE.y);
    snake.push_back(Body {
        x: start_x,
        y: start_y,
    });
    snake.push_back(Body {
        x: start_x,
        y: start_y,
    });
}

fn main() {
    #[cfg(target_os = "windows")]
    thread::spawn(windows::create_hook);

    #[cfg(target_os = "linux")]
    thread::spawn(linux::create_hook);
    init_snake();
    loop {
        show_snake_info();
        println!("Waiting input");
        thread::sleep(Duration::from_secs(1));
        add_snake_head();
        del_snake_tail();
    }
}

fn show_snake_info() {
    println!("{:?}", *DIRECTION.lock().unwrap());
    println!("{:?}", *SNAKE.lock().unwrap());
}

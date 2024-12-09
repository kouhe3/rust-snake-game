use rand::Rng;
use std::sync::Mutex;
use std::{collections::VecDeque, sync::Arc};

#[derive(Clone)]
pub struct Stage {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub x: u16,
    pub y: u16,
}
#[derive(Debug)]
pub struct Food {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
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

pub struct Snake {
    pub body: Mutex<VecDeque<Body>>,
    pub direction: Mutex<Direction>,
}

pub struct Game {
    pub snake: Mutex<Snake>,
    pub player_input: Arc<Mutex<Direction>>,
    pub food: Food,
    pub score: u32,
    pub stage: Stage,
    pub game_over: bool,
}

impl Game {
    pub fn new(x: u16, y: u16) -> Self {
        let mut rng = rand::thread_rng();
        let snake = Snake::new(Stage { x, y });
        let stage = Stage { x, y };
        let food = Food::new(&snake.body, stage.clone());
        let player_input = Arc::new(Mutex::new(Direction::Right));
        Game {
            snake: Mutex::new(snake),
            player_input,
            food,
            score: 0,
            stage,
            game_over: false,
        }
    }

    pub fn step(&mut self) {
        if self.game_over {
            return;
        }
        let mut snake = self.snake.lock().unwrap();
        let prev_snake_body = snake.body.lock().unwrap().clone();
        let binding = Arc::clone(&self.player_input);
        let new_direct = binding.lock().unwrap();
        let new_head = snake.add_head(new_direct.clone());
        //if head is body then game over
        for b in prev_snake_body.iter() {
            if *b == new_head {
                self.game_over = true;
                return;
            }
        }
        //if head hid wall then game over
        if new_head.x == 0 || new_head.y == 0 {
            self.game_over = true;
            return;
        }

        if new_head.x > self.stage.x || new_head.y > self.stage.y {
            self.game_over = true;
            return;
        }

        //if head eat food then do not del_tail
        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.score += 1;
            self.food = Food::new(&snake.body, self.stage.clone());
        } else {
            let _ = snake.del_tail();
        }
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

    //返回新的head的坐标
    fn add_head(&mut self, intput_direction: Direction) -> Body {
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
        snake.push_front(new_head.clone());
        new_head
    }

    //返回删除的尾部坐标
    fn del_tail(&mut self) -> Body {
        let mut snake = self.body.lock().unwrap();
        snake.pop_back().unwrap()
    }
}

impl Food {
    pub fn new(snake_body: &Mutex<VecDeque<Body>>, stage: Stage) -> Self {
        let mut rng = rand::thread_rng();
        //gen food in stage but do not in snake body
        loop {
            let x = rng.gen_range(0..=stage.x);
            let y = rng.gen_range(0..=stage.y);
            let body = snake_body.lock().unwrap();
            if !body.iter().any(|b| b.x == x && b.y == y) {
                return Food { x, y };
            }
        }
    }
}

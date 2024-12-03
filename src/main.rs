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

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

static FOOD: Mutex<Food> = Mutex::new(Food { x: 0, y: 0 });
static STAGE: Stage = Stage { x: 100, y: 100 };
static DIRECTION: LazyLock<Arc<Mutex<Direction>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Direction::Up)));
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

#[allow(non_snake_case)]
unsafe extern "system" fn LowLevelKeyboardProc(
    nCode: i32,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    if nCode as u32 == HC_ACTION {
        let kb_data = unsafe { *(lParam.0 as *mut KBDLLHOOKSTRUCT) };
        if wParam.0 as u32 == WM_KEYDOWN {
            match kb_data.vkCode {
                0x25 => *DIRECTION.lock().unwrap() = Direction::Left,
                0x26 => *DIRECTION.lock().unwrap() = Direction::Up,
                0x27 => *DIRECTION.lock().unwrap() = Direction::Right,
                0x28 => *DIRECTION.lock().unwrap() = Direction::Down,
                _ => {}
            }
        }
    }

    unsafe { CallNextHookEx(None, nCode, wParam, lParam) }
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
    thread::spawn(create_hook);
    init_snake();
    loop {
        show_snake_info();
        println!("Waiting input");
        thread::sleep(Duration::from_secs(1));
        add_snake_head();
        del_snake_tail();
    }
}

fn create_hook() {
    //start hook for listen keyboard input
    let hh = unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(LowLevelKeyboardProc), None, 0) };
    if hh.is_err() {
        panic!("hook error");
    }
    //msg loop
    unsafe {
        let mut msg = std::mem::zeroed();
        while GetMessageA(&mut msg, None, 0, 0).0 != 0 {
            TranslateMessage(&mut msg);
            DispatchMessageA(&mut msg);
        }
    }
}

fn show_snake_info() {
    println!("{:?}", *DIRECTION.lock().unwrap());
    println!("{:?}", *SNAKE.lock().unwrap());
}

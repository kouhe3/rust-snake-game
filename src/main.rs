use tanchishe::game::Game;
use tanchishe::render::show_debug_info;
use tanchishe::listen_key::listen_keyboard;
use std::{time::Duration,thread,sync::{Arc, Mutex}};

fn main() -> std::io::Result<()> {
    let mut game = Game::new(100, 100);
    let player_input_clone = Arc::clone(&game.player_input);
    let keyboard_thread =  thread::spawn(|| listen_keyboard(player_input_clone));
    
    loop {
        show_debug_info(&game);
        game.step();
        thread::sleep(Duration::from_millis(500));
    }
    //keyboard_thread.join();
    Ok(())
}

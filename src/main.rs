use tanchishe::game::Game;
use tanchishe::render::*;
use tanchishe::listen_key::listen_keyboard;
use std::{time::Duration,thread,sync::{Arc, Mutex}};

fn main() -> std::io::Result<()> {
    let mut game = Game::new(50, 20);
    let player_input_clone = Arc::clone(&game.player_input);
    let keyboard_thread =  thread::spawn(|| listen_keyboard(player_input_clone));
    
    loop {
        thread::sleep(Duration::from_millis(500));
        //show_debug_info(&game);
        game.step();
        game_display(&game);
        
    }
    //keyboard_thread.join();
    Ok(())
}

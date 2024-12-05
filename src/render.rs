use crate::game::Game;
pub fn show_debug_info(game:&Game){
    println!("===========================");
    //show snake body info
    println!("snake body info:");
    let g = game.snake.lock().unwrap();
    let b = g.body.lock().unwrap();
    println!("{:?}",b);
    println!("snake direct:");
    let d = g.direction.lock().unwrap();
    println!("{:?}",d);
    println!("Food:");
    println!("{:?}",game.food);
    println!("Player input:");
    let p = game.player_input.lock().unwrap();
    println!("{:?}",p);
    println!("Score:");
    println!("{}",game.score);
    println!("Game over:");
    println!("{}",game.game_over);


}
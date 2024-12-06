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

pub fn game_display(game:&Game){
    print!("\x1B[2J");
    for stage_y in 0..=game.stage.y{
        for stage_x in 0..=game.stage.x{
            //check if this block is snake body
            let mut is_body = false;
            let g = game.snake.lock().unwrap();
            let b = g.body.lock().unwrap();
            for bodys in b.iter() {
                if bodys.x == stage_x && bodys.y == stage_y {
                    is_body = true;
                    break;
                }
            }
            
            //then check if this block is food
            let mut is_food = false;
            if game.food.x == stage_x && game.food.y == stage_y {
                is_food = true;
                
            }

            if is_body {
                print!("#");
            }else if is_food {
                print!("*");
            }else {
                print!(" ");
            }

        }
        print!("||\n");
    }
    for i in 0..game.stage.x {
        print!("=");
    }
    println!("");
    if game.game_over{
        println!("You die");
    }


}
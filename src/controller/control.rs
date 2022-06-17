// #![allow(unused_imports)]

use std::io::stdout;
use crate::game::game::Game;


pub fn controller_fn(){

    Game::new(stdout(), 10, 10).run();
    println!("controller_fn");

}
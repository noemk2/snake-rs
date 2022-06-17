
#![allow(unused_imports)]

mod snake;
mod direction;
mod game;
mod point;
mod command;

use crate::game::Game;
use std::io::stdout;


pub fn controller_fn(){

    Game::new(stdout(), 10, 10).run();
    println!("controller_fn");

}
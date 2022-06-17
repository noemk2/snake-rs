// use crate::direction::Direction;

use crate::snake::direction::Direction;


pub enum Command {
    Quit,
    Turn(Direction),
}
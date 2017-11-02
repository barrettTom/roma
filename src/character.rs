extern crate rand;
use character::rand::Rng;

use location::Location;

pub struct Character{
    pub symbol      : char,
    pub color       : u8,
    pub location    : Location,
}

impl Copy for Character {}
impl Clone for Character {
    fn clone(&self) -> Character {
        *self
    }
}

impl Character {
    pub fn new(symbol : char, color : u8, location : Location) -> Character {
        Character {
            symbol      : symbol,
            color       : color,
            location    : location,
        }
    }

    pub fn action(&mut self, free_spaces : Vec<Location>) {
        self.wander(free_spaces);
    }

    fn wander(&mut self, free_spaces : Vec<Location>) {
        let direction = rand::thread_rng().gen_range(0, free_spaces.len());
        self.location = free_spaces[direction];
    }
}

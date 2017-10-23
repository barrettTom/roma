extern crate pancurses;
use pancurses::ColorPair;

extern crate rand;
use character::rand::Rng;

use location::Location;

pub struct Character{
    symbol : char,
    color  : u8,
    pub location : Location,
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
            symbol : symbol,
            color : color,
            location : location,
        }
    }

    pub fn draw(self, window : &pancurses::Window) {
        window.attron(ColorPair(self.color));
        window.mvaddch(self.location.x, self.location.y, self.symbol);
    }

    pub fn action(self, men : Vec<Character>, impassable : Vec<Location>){
        self.wander(men, impassable);
    }

    fn wander(mut self, men : Vec<Character>, impassable : Vec<Location>){
        let direction = rand::thread_rng().gen_range(0,9);
        let mut desired_location = self.location;
        if direction == 0 {
            desired_location.x = desired_location.x+1;
        }

        if self.free_space(desired_location, men, impassable) {
            self.location = desired_location;
        }
    }

    fn free_space(self, location : Location, men : Vec<Character>, impassable : Vec<Location>) -> bool {
        true
    }
}

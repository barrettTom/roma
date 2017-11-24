extern crate rand;
use character::rand::Rng;


use constants::Orders;
use location::Location;

#[derive(Clone)]
pub struct Character {
    symbol              : char,
    color               : u8,
    order               : u8,
    location            : Location,
    desired_location    : Option<Location>,
    path                : Option<Vec<Location>>,
}

impl Character {

    pub fn new(symbol : char, color : u8, location : Location) -> Character {
        Character {
            symbol              : symbol,
            color               : color,
            order               : Orders::Wander as u8,
            location            : location,
            desired_location    : None,
            path                : None,
        }
    }


    pub fn action(&mut self, free_spaces : Vec<(Location,usize)>) {
        if self.order == Orders::Wander as u8 {
            self.wander(free_spaces);
        }
        else if self.order == Orders::Move as u8 {
            self.move_along_path(free_spaces);
        }
    }

    pub fn up(&mut self) {
        self.location.0 -= 1;
    }

    pub fn down(&mut self) {
        self.location.0 += 1;
    }

    pub fn right(&mut self) {
        self.location.1 += 1;
    }

    pub fn left(&mut self) {
        self.location.1 -= 1;
    }

    pub fn get_symbol(&self) -> char {
        self.symbol
    }

    pub fn get_color(&self) -> u8 {
        self.color
    }

    pub fn get_location(&self) -> Location {
        self.location
    }

    pub fn get_desired_location(&self) -> Option<Location> {
        self.desired_location
    }

    pub fn give_path(&mut self, path : Option<Vec<Location>>) {
        match path {
            Some(path) => {
                self.path = Some(path);
            },
            None => {
                self.path = None;
                self.order = Orders::Wander as u8;
            },
        }
    }

    pub fn give_destination(&mut self, destination : Location) {
        self.desired_location = Some(destination);
        self.order = Orders::Move as u8;
    }

    pub fn needs_path(&self) -> bool {
        if self.order == Orders::Wander as u8 {
            false
        }
        else if self.path.is_some() {
            false
        }
        else if self.desired_location.is_some() && self.path.is_none() {
            true
        }
        else {
            false
        }
    }

    fn wander(&mut self, free_spaces : Vec<(Location, usize)>) {
        let direction = rand::thread_rng().gen_range(0, free_spaces.len());
        self.location = free_spaces[direction].0;
    }

    fn move_along_path(&mut self, free_spaces : Vec<(Location, usize)>) {
        let mut moved = false;
        match self.path {
            None => (),
            Some(ref mut calculated_path) => {
                if calculated_path.len() > 0 {
                    let next_location = calculated_path.pop().unwrap();
                    for free_space in free_spaces {
                        if next_location == free_space.0 {
                            self.location = next_location;
                            moved = true;
                        }
                    }
                }
                else {
                    self.desired_location = None;
                    self.order = Orders::Wait as u8;
                }
            }
        }
        if !moved {
            self.path = None;
        }
    }
}

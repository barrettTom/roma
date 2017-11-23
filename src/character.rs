extern crate rand;
use character::rand::Rng;

extern crate pathfinding;
use self::pathfinding::astar;

use constants::Orders;
use location::Location;

#[derive(Clone)]
pub struct Character {
    pub symbol          : char,
    pub color           : u8,
    pub order           : u8,
    pub location        : Location,
    pub needs_path      : bool,
    desired_location    : Option<Location>,
    path                : Option<Vec<Location>>,
}

impl Character {
    pub fn new(symbol : char, color : u8, location : Location) -> Character {
        Character {
            symbol              : symbol,
            color               : color,
            order               : Orders::Move as u8,
            location            : location,
            desired_location    : Some(Location(200,200)),
            needs_path          : false,
            path                : None,
        }
    }

    pub fn action(&mut self, free_spaces : Vec<(Location,usize)>) {
        if self.order == Orders::Wander as u8 {
            self.needs_path == false;
            self.wander(free_spaces);
        }
        else if self.order == Orders::Move as u8 {
            self.move_toward_desired(free_spaces);
        }
    }

    fn wander(&mut self, free_spaces : Vec<(Location, usize)>) {
        let direction = rand::thread_rng().gen_range(0, free_spaces.len());
        self.location = free_spaces[direction].0;
    }

    fn move_toward_desired(&mut self, free_spaces : Vec<(Location, usize)>) {
        let mut moved = false;
        match self.path {
            None => self.needs_path = true,
            Some(ref mut calculated_path) => {
                self.needs_path = false;
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
                    self.order = Orders::Wander as u8;
                }
            }
        }
        if !moved {
            self.path = None;
        }
    }

    pub fn calculate_path(&mut self, impassable : Vec<(Location, usize)>) {
        match self.desired_location {
            None => (),
            Some(target) => {
                let location = self.location;
                let result = astar(&location,
                                       |l| l.neighbours(impassable.clone()),
                                       |l| l.distance(&target),
                                       |l| *l == target);
                let mut result = result.expect("zz").0;
                result.reverse();
                result.pop();
                self.path = Some(result);
            }
        }
    }
}

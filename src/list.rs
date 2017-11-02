extern crate pancurses;

use location::Location;
use character::Character;
use constants::Colors;

pub struct List{
    pub men                 : Vec<Character>,
    impassable_locations    : Vec<Location>,
}

impl List {
    pub fn new(impassable_locations : Vec<Location>) -> List {
        let mut men = Vec::new();
        for i in 0..3 {
            men.push(Character::new('@', Colors::BlueUnit as u8, Location{ x : 150, y : 150+i }));
        }
        List {
            men                     : men,
            impassable_locations    : impassable_locations,
        }
    }

    pub fn action(&mut self) {
        for i in 0..self.men.len() {
            let location = self.men[i].location.clone();
            let free_locations = self.get_free_locations(location);
            self.men[i].action(free_locations);
        }
    }

    fn get_free_locations(&mut self, location : Location) -> Vec<Location> {
        let mut potential_locations = location.get_around();

        potential_locations.retain(|potential_location| {
            let mut keep = true;
            for man in self.men.iter() {
                if potential_location == &man.location {
                    keep = false;
                }
            }
            for impassable_location in self.impassable_locations.iter() {
                if potential_location == impassable_location {
                    keep = false;
                }
            }
            keep
        });

        potential_locations
    }
}

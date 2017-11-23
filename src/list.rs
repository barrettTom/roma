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
            men.push(Character::new('@', Colors::BlueUnit as u8, Location(150,150+i)));
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

            if self.men[i].needs_path {
                let impassable = self.get_all_impassable();
                self.men[i].calculate_path(impassable);
            }
        }
    }

    pub fn give_destination(&mut self, destination : Location) {
        for i in 0..self.men.len() {
            self.men[i].give_destination(destination)
        }
    }

    fn get_free_locations(&mut self, location : Location) -> Vec<(Location, usize)> {
        let mut potential_locations = location.neighbours(Vec::new());

        potential_locations.retain(|potential_location| {
            let mut keep = true;
            for man in self.men.iter() {
                if potential_location.0 == man.location {
                    keep = false;
                }
            }
            for impassable_location in self.impassable_locations.iter() {
                if potential_location.0 == *impassable_location {
                    keep = false;
                }
            }
            keep
        });

        potential_locations
    }

    fn get_all_impassable(&mut self) -> Vec<(Location, usize)> {
        let mut impassable = Vec::new();
        for man in self.men.iter() {
            impassable.push((man.location, 1));
        }
        for impassable_location in self.impassable_locations.iter() {
            impassable.push((*impassable_location,1));
        }
        impassable
    }
}

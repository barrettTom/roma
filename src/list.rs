extern crate pancurses;

use character::Character;
use location::Location;

pub struct List{
    pub men                 : Vec<Character>,
    impassable_locations    : Vec<Location>,
}

impl List {
    pub fn new(impassable_locations : Vec<Location>) -> List {
        let mut men = Vec::new();
        for i in 0..3 {
            let l = Location{x:150,y:150+i};
            let c = Character::new('@',4,l);
            men.push(c);
        }
        List {
            men                     : men,
            impassable_locations    : impassable_locations,
        }
    }

    pub fn action(&mut self) {
        for i in 0..self.men.len() {
            let tmp = self.men[i].location.clone();
            let free_locations = self.get_free_locations(tmp);
            self.men[i].action(free_locations);
        }
    }

    fn get_free_locations(&mut self, location : Location) -> Vec<Location> {
        let mut potential_locations = Vec::new();
        potential_locations.push(location.up());
        potential_locations.push(location.upleft());
        potential_locations.push(location.upright());
        potential_locations.push(location.down());
        potential_locations.push(location.downleft());
        potential_locations.push(location.downright());
        potential_locations.push(location.left());
        potential_locations.push(location.right());

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

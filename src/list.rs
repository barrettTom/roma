use std::thread::{spawn, JoinHandle};

extern crate pancurses;

extern crate pathfinding;
use self::pathfinding::astar;

use location::Location;
use character::Character;
use constants::Colors;

pub struct List{
    pub men                 : Vec<Character>,
    impassable_locations    : Vec<Location>,
    threads                 : Vec<JoinHandle<(usize, Option<Vec<Location>>)>>
}

impl List {

    pub fn new(impassable_locations : Vec<Location>) -> List {
        let mut men = Vec::new();
        for i in 0..10 {
            men.push(Character::new('@', Colors::BlueUnit as u8, Location(150,150+i)));
        }
        List {
            men                     : men,
            impassable_locations    : impassable_locations,
            threads                 : Vec::new(),
        }
    }

    pub fn action(&mut self) {
        for i in 0..self.men.len() {
            let location = self.men[i].get_location();
            let free_locations = self.get_free_locations(location);
            self.men[i].action(free_locations);
        }

        while !self.threads.is_empty() {
            let thread = self.threads.pop().unwrap();
            let (i, path) = thread.join().unwrap();
            self.men[i].give_path(path);
        }

        let impassable = self.get_all_impassable();
        for i in 0..self.men.len() {
            if self.men[i].needs_path() {
                let man = self.men[i].clone();
                let impassable_clone = impassable.clone();
                self.threads.push(spawn(move || {
                    (i, calculate_path(man, impassable_clone))
                }));
            }
        }
    }

    pub fn give_destination(&mut self, destination : Location) {
        for i in 0..self.men.len() {
            self.men[i].give_destination(destination)
        }
    }

    pub fn give_grid(&mut self, first_location : Location, second_location : Location) {
        let mut index = 0;
        for i in first_location.0..second_location.0 + 1 {
            for j in first_location.1..second_location.1 + 1 {
                if index < self.men.len() {
                    self.men[index].give_destination(Location(i,j));
                    index += 1;
                }
            }
        }
        for i in second_location.0..first_location.0 {
            for j in second_location.1..first_location.1 {
                if index < self.men.len() {
                    self.men[index].give_destination(Location(i,j));
                    index += 1;
                }
            }
        }
        for i in first_location.0..second_location.0 + 1 {
            for j in second_location.1..first_location.1 {
                if index < self.men.len() {
                    self.men[index].give_destination(Location(i,j));
                    index += 1;
                }
            }
        }
        for i in second_location.0..first_location.0 {
            for j in first_location.1..second_location.1 + 1 {
                if index < self.men.len() {
                    self.men[index].give_destination(Location(i,j));
                    index += 1;
                }
            }
        }
    }

    fn get_free_locations(&mut self, location : Location) -> Vec<(Location, usize)> {
        let mut potential_locations = location.neighbours(Vec::new());

        potential_locations.retain(|potential_location| {
            let mut keep = true;
            for man in self.men.iter() {
                if potential_location.0 == man.get_location() {
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
            impassable.push((man.get_location(), 1));
        }
        for impassable_location in self.impassable_locations.iter() {
            impassable.push((*impassable_location,1));
        }
        impassable
    }
}

fn calculate_path(man : Character, impassable : Vec<(Location, usize)>) -> Option<Vec<Location>> {
    let desired_location = man.get_desired_location();
    match desired_location {
        Some(target) => {
            let location = man.get_location();
            let result = astar(&location,
                               |l| l.neighbours(impassable.clone()),
                               |l| l.distance(&target),
                               |l| *l == target);
            match result {
                Some(mut result) => {
                    result.0.reverse();
                    result.0.pop();
                    Some(result.0)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

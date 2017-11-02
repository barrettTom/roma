use std::cmp;

pub struct Location {
    pub x : i32, 
    pub y : i32
}

impl Copy for Location {}
impl Clone for Location {
    fn clone(&self) -> Location {
        *self
    }
}

impl cmp::PartialEq for Location {
    fn eq(&self, rhs : &Location) -> bool {
        if self.x == rhs.x && self.y == rhs.y {
            true
        }
        else {
            false
        }
    }
}

impl Location {
    pub fn get_around(self) -> Vec<Location> {
        let mut around = Vec::new();
        around.push(Location { x : self.x, y : self.y });
        around.push(Location { x : self.x+1, y : self.y });
        around.push(Location { x : self.x-1, y : self.y });
        around.push(Location { x : self.x, y : self.y+1 });
        around.push(Location { x : self.x, y : self.y-1 });
        around.push(Location { x : self.x+1, y : self.y+1 });
        around.push(Location { x : self.x-1, y : self.y-1 });
        around.push(Location { x : self.x+1, y : self.y-1 });
        around.push(Location { x : self.x-1, y : self.y+1 });
        around
    }
}

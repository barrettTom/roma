use std::cmp;

pub struct Location{
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
    pub fn up(mut self) -> Location {
        self.y += 1;
        self
    }
    pub fn down(mut self) -> Location {
        self.y -= 1;
        self
    }
    pub fn right(mut self) -> Location {
        self.x += 1;
        self
    }
    pub fn left(mut self) -> Location {
        self.x -= 1;
        self
    }
}

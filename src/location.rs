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
    pub fn up(self) -> Location {
        Location{ x : self.x, y : self.y + 1 }
    }
    pub fn upleft(self) -> Location {
        Location{ x : self.x - 1, y : self.y + 1 }
    }
    pub fn upright(self) -> Location {
        Location{ x : self.x + 1, y : self.y + 1 }
    }
    pub fn down(self) -> Location {
        Location{ x : self.x, y : self.y - 1 }
    }
    pub fn downleft(self) -> Location {
        Location{ x : self.x - 1, y : self.y - 1 }
    }
    pub fn downright(self) -> Location {
        Location{ x : self.x + 1, y : self.y - 1 }
    }
    pub fn right(self) -> Location {
        Location{ x : self.x + 1, y : self.y }
    }
    pub fn left(self) -> Location {
        Location{ x : self.x - 1, y : self.y }
    }
}

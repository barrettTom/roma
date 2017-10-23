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

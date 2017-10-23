extern crate pancurses;

use character::Character;
use location::Location;

pub struct List{
    men : Vec<Character>,
    impassable : Vec<Location>,
}

impl List{
    pub fn new(impassable : Vec<Location>) -> List {
        let mut men = Vec::new();
        for i in 0..3 {
            let l = Location{x:150,y:150+i};
            let c = Character::new('@',4,l);
            men.push(c);
        }
        List{
            men : men,
            impassable : impassable,
        }
    }

    pub fn draw(&self, window : &pancurses::Window) {
        for man in self.men.iter(){
            man.draw(window);
        }
    }

    pub fn action(&self) {
        for man in self.men.iter(){
            man.action(self.men.to_vec(), self.impassable.to_vec());
        }
    }
}

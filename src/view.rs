extern crate pancurses;

use character::Character;

pub struct View {
	width   : i32,
	height  : i32,
    row     : i32,
    col     : i32,
	window  : pancurses::Window,
}

impl View {
    pub fn new((x,y) : (i32, i32), map_window : &pancurses::Window) -> View {
        let window = map_window.derwin(x,y,0,0).expect("Cannot create derwin.");
        View {
            width   : x,
            height  : y,
            row     : 0,
            col     : 0, 
            window  : window,
        }
    }

    pub fn center(&mut self, character : Character, map_window : &pancurses::Window) {
        let location = character.get_location();
        let c = location.0 - self.width/2;
        let r = location.1 - self.height/2;

        let (hh, ww) = map_window.get_max_yx();
        
        if c + self.width >= ww {
            let delta = ww - (c + self.width);
            self.col = c + delta;
        }
        else {
            self.col = c;
        }

        if r + self.height >= hh {
            let delta = hh - (r + self.height);
            self.row = r + delta;
        }
        else {
            self.row = r;
        }

        if r < 0 {
            self.row = 0;
        }

        if c < 0 {
            self.col = 0;
        }
        
        self.window.mvderwin(self.col, self.row);
        map_window.touch();
        self.window.refresh();
    }
}

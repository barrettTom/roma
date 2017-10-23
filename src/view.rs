extern crate pancurses;

use character::Character;

pub struct View{
	width: 		    i32,
	height: 	    i32,
    row:            i32,
    col:            i32,
	window:		    pancurses::Window,
}

impl View{
    pub fn new((x,y) : (i32, i32), map_window : &pancurses::Window) -> View {
        let window = map_window.derwin(x,y,0,0).expect("help");
        View{
            width : 	    x,
            height : 	    y,
            row :           0,
            col :           0, 
            window:		    window,
        }
    }

    pub fn center(&mut self, character : Character, (hh, ww) : (i32,i32)) {
        let mut rr = self.row;
        let mut cc = self.col;
        let r = character.location.x - self.height/2;
        let c = character.location.y - self.width/2;
        
        if c + self.width >= ww {
            let delta = ww - (c + self.width);
            cc = c + delta;
        }
        else {
            cc = c;
        }

        if r + self.height >= hh {
            let delta = hh - (r + self.height);
            rr = r + delta;
        }
        else {
            rr = r;
        }

        if r < 0 {
            rr = 0;
        }

        if c < 0 {
            cc = 0;
        }
        
        self.window.refresh();
        self.window.mv(rr, cc);
        self.row = rr;
        self.col = cc;
        self.window.refresh();
    }
}

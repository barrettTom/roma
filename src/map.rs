use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate pancurses;
use pancurses::{newwin, ColorPair};

use character::Character;
use location::Location;

pub struct Map{
    pub height :    i32,
    pub width :     i32,
	pub window :	pancurses::Window,
	map_data :      Vec<String>,
    pub impassable :Vec<Location>,
}

impl Map {
    pub fn new() -> Map {
        let file = File::open("data/map.txt").expect("Cant open map file !");
        let reader = BufReader::new(file);

        let mut map_data = Vec::new();
        for line in reader.lines(){
            map_data.push(line.unwrap());
        }

        let height = map_data.len() as i32;
        let width = map_data[0].len() as i32;

        let mut impassable = Vec::new();
        for (i, row) in map_data.iter().enumerate() {
            for (j, index) in row.chars().enumerate() {
                match index {
                    '0' | 'O' => impassable.push(Location{x : i as i32, y : j as i32}),
                    _ => (),
                }
            }
        }

        for y in 0..height {
            impassable.push(Location{x : 0 as i32, y : y as i32});
            impassable.push(Location{x : width-1 as i32, y : y as i32});
        }

        for x in 0..width {
            impassable.push(Location{x : x as i32, y : 0 as i32});
            impassable.push(Location{x : x as i32, y : height-1 as i32});
        }

        Map {
            height : 	    height,
            width : 	    width,
            window:		    newwin(height, width, 0, 0),
            map_data:	    map_data,
            impassable:     impassable,
        }
    }

    pub fn draw(&self, character : &Character) {
        self.window.attron(ColorPair(character.color));
        self.window.mvaddch(character.location.x, character.location.y, character.symbol);
    }

    pub fn fill(&mut self) {
        for (i, row) in self.map_data.iter().enumerate() {
            for (j, index) in row.chars().enumerate() {
                match index {
                    '0' | 'O' => {
                        self.window.attron(ColorPair(2));
                        self.window.mvaddch(i as i32, j as i32, index);
                    }
                    _ => {
                        self.window.attron(ColorPair(1));
                        self.window.mvaddch(i as i32, j as i32, index);
                    }
                }
            }
        }

        self.window.attron(ColorPair(3));
        for y in 0..self.height {
            self.window.mvaddch(y, 0, '-');
            self.window.mvaddch(y, self.width-1, '-');
        }

        for x in 0..self.width {
            self.window.mvaddch(0, x, '|');
            self.window.mvaddch(self.height-1, x, '|');
        }
    }
}

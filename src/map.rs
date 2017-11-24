use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate pancurses;
use pancurses::{newwin, ColorPair};

use location::Location;
use character::Character;
use constants::Colors;

pub struct Map {
    pub height      : i32,
    pub width       : i32,
	pub window      : pancurses::Window,
	map_data        : Vec<String>,
    pub impassable  : Vec<Location>,
}

impl Map {
    pub fn new() -> Map {
        let file = File::open("data/map.txt").expect("Cant open map file !");
        let reader = BufReader::new(file);

        let mut map_data = Vec::new();
        for line in reader.lines() {
            map_data.push(line.unwrap());
        }

        let height = map_data.len() as i32;
        let width = map_data[0].len() as i32;

        let mut impassable = Vec::new();
        for (i, row) in map_data.iter().enumerate() {
            for (j, index) in row.chars().enumerate() {
                match index {
                    '0' | 'O' => impassable.push(Location(i as i32, j as i32)),
                    _ => (),
                }
            }
        }

        for y in 0..height {
            impassable.push(Location(0, y as i32));
            impassable.push(Location(width-1 as i32, y as i32));
        }

        for x in 0..width {
            impassable.push(Location(x as i32, 0 as i32));
            impassable.push(Location(x as i32, height-1));
        }

        Map {
            height      : height,
            width       : width,
            window      : newwin(height, width, 0, 0),
            map_data    : map_data,
            impassable  : impassable,
        }
    }

    pub fn draw(&self, character : &Character) {
        self.window.attron(ColorPair(character.color));
        self.window.mvaddch(character.location.0, character.location.1, character.symbol);
    }

    pub fn draw_box(&self, first_location : Location, second_location : Location) {
        for i in first_location.0..second_location.0 + 1 {
            for j in first_location.1..second_location.1 + 1 {
                self.window.attron(ColorPair(Colors::White as u8));
                self.window.mvaddch(i as i32, j as i32, 'X');
            }
        }
        for i in second_location.0..first_location.0 {
            for j in second_location.1..first_location.1 {
                self.window.attron(ColorPair(Colors::White as u8));
                self.window.mvaddch(i as i32, j as i32, 'X');
            }
        }
        for i in first_location.0..second_location.0 + 1 {
            for j in second_location.1..first_location.1 {
                self.window.attron(ColorPair(Colors::White as u8));
                self.window.mvaddch(i as i32, j as i32, 'X');
            }
        }
        for i in second_location.0..first_location.0 {
            for j in first_location.1..second_location.1 + 1 {
                self.window.attron(ColorPair(Colors::White as u8));
                self.window.mvaddch(i as i32, j as i32, 'X');
            }
        }
    }

    pub fn fill(&mut self) {
        for (i, row) in self.map_data.iter().enumerate() {
            for (j, index) in row.chars().enumerate() {
                match index {
                    '0' | 'O' => {
                        self.window.attron(ColorPair(Colors::Tree as u8));
                        self.window.mvaddch(i as i32, j as i32, index);
                    }
                    _ => {
                        self.window.attron(ColorPair(Colors::Grass as u8));
                        self.window.mvaddch(i as i32, j as i32, index);
                    }
                }
            }
        }

        self.window.attron(ColorPair(Colors::White as u8));
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

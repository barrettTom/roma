extern crate pancurses;
use pancurses::{initscr, endwin, noecho, start_color, Input, init_pair, COLOR_YELLOW, COLOR_BLACK, COLOR_WHITE, COLOR_BLUE, COLOR_GREEN, cbreak, curs_set};

mod map;
mod view;
mod list;
mod location;
mod character;
mod constants;

use map::Map;
use view::View;
use list::List;
use location::Location;
use character::Character;
use constants::{Colors, Commands};

fn init() -> pancurses::Window {
    let main = initscr();
    main.clear();
	noecho();
    cbreak();
    curs_set(0);
    main.timeout(100);
    start_color();
    init_pair(Colors::Grass as i16, COLOR_GREEN, COLOR_BLACK);
    init_pair(Colors::Tree as i16, COLOR_YELLOW, COLOR_BLACK);
    init_pair(Colors::White as i16, COLOR_WHITE, COLOR_WHITE);
    init_pair(Colors::BlueUnit as i16, COLOR_WHITE, COLOR_BLUE);
    main
}

fn main() {

    let main = init();
    let mut map = Map::new();
    let mut view = View::new(main.get_max_yx(), &map.window);

    let mut cursor = Character::new('X', Colors::White as u8, Location(150, 150));

    let mut list = List::new(map.impassable.to_vec());

    let mut paused = false;
    let mut first_location = None;
    let mut draw_box = false;
    loop{
        let command = match main.getch() {
            Some(Input::Character(ch)) => {
                match ch {
                    'k' => {cursor.up();   None}
                    'j' => {cursor.down(); None}
                    'h' => {cursor.left(); None}
                    'l' => {cursor.right();None}
                    'q' => break,
                    'o' => Some(Commands::Go),
                    's' => Some(Commands::Grid),
                   '\n' => Some(Commands::Finish),
                    _ => None,
                }
            },
            Some(Input::KeyEnter) => Some(Commands::Finish),
            _ => None,
        };

        map.fill();

        match command {
            Some(Commands::Go) => list.give_destination(cursor.get_location()),
            Some(Commands::Grid) => {
                paused = true;
                draw_box = true;
                first_location = Some(cursor.get_location());
            },
            Some(Commands::Finish) => {
                paused = false;
                draw_box = false;
                match first_location {
                    Some(first_location) => list.give_grid(first_location, cursor.get_location()),
                    None => (),
                }
            },
            _ => (),
        }

        if !paused {
            list.action();
        }

        map.draw(&cursor);

        if draw_box {
            match first_location {
                Some(first_location) => map.draw_box(first_location, cursor.get_location()),
                _ => (),
            }
        }

        for man in list.men.iter() {
            map.draw(man);
        }

        view.center(cursor.clone(), &map.window);
    }

	endwin();
}

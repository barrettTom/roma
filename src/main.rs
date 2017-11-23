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
use constants::Colors;

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

    loop{
        let order = match main.getch() {
            Some(Input::Character(c)) => {
                match c {
                    'h' => {cursor.location.1 -= 1; None}
                    'l' => {cursor.location.1 += 1; None}
                    'k' => {cursor.location.0 -= 1; None}
                    'j' => {cursor.location.0 += 1; None}
                    'q' => break,
                    'o' => Some(cursor.location),
                    _ => None,
                }
            },
            _ => None
        };

        match order {
            None => (),
            Some(location) => (list.give_destination(location)),
        }

        list.action();

        map.fill();

        for man in list.men.iter() {
            map.draw(man);
        }
        map.draw(&cursor);

        view.center(cursor.clone(), &map.window);
    }

	endwin();
}

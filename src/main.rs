extern crate pancurses;
use pancurses::{initscr, endwin, noecho, start_color, Input, init_pair, COLOR_YELLOW, COLOR_BLACK, COLOR_WHITE, COLOR_BLUE, COLOR_GREEN, cbreak, curs_set};

mod map;
mod view;
mod list;
mod character;
mod location;

use map::Map;
use view::View;
use list::List;
use character::Character;
use location::Location;

fn init() -> pancurses::Window {
    let main = initscr();
    main.clear();
	noecho();
    cbreak();
    curs_set(0);
    main.timeout(100);
    start_color();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    init_pair(2, COLOR_YELLOW, COLOR_BLACK);
    init_pair(3, COLOR_WHITE, COLOR_WHITE);
    init_pair(4, COLOR_WHITE, COLOR_BLUE);
    main
}

fn main() {

    let main = init();
    let mut map = Map::new();
    let mut view = View::new(main.get_max_yx(), &map.window);

    let mut cursor = Character::new('X', 3, Location{x:150,y:150});

    let list = List::new(map.impassable.to_vec());

    let paused = false;
    loop{
        match main.getch() {
            Some(Input::Character(c)) => {
                match c {
                    'h' => cursor.location.y -= 1,
                    'l' => cursor.location.y += 1,
                    'k' => cursor.location.x -= 1,
                    'j' => cursor.location.x += 1,
                    'q' => break,
                    _ => (),
                }
            },
            _ => ()
        }

        if !paused {
            list.action();
        }

        map.fill();
        list.draw(&map.window);
        cursor.draw(&map.window);
        view.center(cursor, &map.window);
    }

	endwin();
}

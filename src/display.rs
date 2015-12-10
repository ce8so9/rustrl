extern crate ncurses;

use ncurses::*;

pub struct Display {
    items_to_display : Vec<i32>
}

impl Display {

    pub fn new() -> Display {
        Display {
            items_to_display : vec![1, 2, 3, 4, 5]
        }
    }

    pub fn setup(&self) {
        initscr();
        start_color();
        noecho();
        cbreak();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        keypad(stdscr, true);
    }

    pub fn cleanup(&self) {
        nocbreak();
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        keypad(stdscr, false);
        echo();
        endwin();
    }
}


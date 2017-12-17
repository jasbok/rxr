extern crate ncurses;

use std::collections::HashMap;
use std::path::PathBuf;

pub struct Menu<'a> {
    paths: &'a Vec<PathBuf>,
}

impl<'a> Menu<'a> {
    pub fn from(paths: &'a Vec<PathBuf>) -> Menu<'a> {
        Menu::<'a> { paths: paths }
    }

    pub fn display(&self) -> usize {
        ncurses::initscr();

        let chars = vec![
            b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'q', b'w', b'e', b'r',
            b't', b'y', b'u', b'i', b'o', b'p', b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k',
            b'l', b';', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/',
        ];
        let mut options: HashMap<u8, usize> = HashMap::new();

        let mut selection: Option<usize> = None;
        while selection == None {
            for (i, path) in self.paths.iter().enumerate() {
                if i >= chars.len() {
                    break;
                }

                ncurses::printw(&format!("[{}] {:#?}\n", chars[i] as char, path));
                options.insert(chars[i], i);
            }

            ncurses::refresh();

            let select = options.get(&(ncurses::getch() as u8));

            if select != None {
                selection = Some(select.unwrap().clone());
            }

            ncurses::clear();
            ncurses::refresh();
        }

        ncurses::endwin();
        selection.unwrap()
    }
}

extern crate ncurses;

use std::collections::HashMap;
use std::path::PathBuf;

pub struct Menu<'a> {
    paths: &'a [PathBuf],
}

impl<'a> Menu<'a> {
    pub fn from(paths: &'a [PathBuf]) -> Menu<'a> {
        Menu::<'a> { paths: paths }
    }

    pub fn display(&self) -> &PathBuf {
        ncurses::initscr();

        let chars = vec![
            b'1',
            b'2',
            b'3',
            b'4',
            b'5',
            b'6',
            b'7',
            b'8',
            b'9',
            b'0',
            b'q',
            b'w',
            b'e',
            b'r',
            b't',
            b'y',
            b'u',
            b'i',
            b'o',
            b'p',
            b'a',
            b's',
            b'd',
            b'f',
            b'g',
            b'h',
            b'j',
            b'k',
            b'l',
            b';',
            b'z',
            b'x',
            b'c',
            b'v',
            b'b',
            b'n',
            b'm',
            b',',
            b'.',
            b'/',
        ];

        let mut selection: Option<&PathBuf> = None;
        let mut page = 0;
        let page_size = 10;
        let pages: usize = self.paths.len() / page_size;

        while selection == None {
            let mut options: HashMap<u8, &PathBuf> = HashMap::new();
            let start = page * page_size;
            let mut end = start + page_size;

            if end > self.paths.len() {
                end = self.paths.len();
            }

            ncurses::printw(&format!(
                "Select Executable (Page {} of {})\n",
                page + 1,
                pages + 1
            ));
            for (i, path) in self.paths.iter().enumerate() {
                if start <= i && i < end {
                    let index = i - start;
                    ncurses::printw(&format!("[{}] {:#?}\n", chars[index] as char, path));
                    options.insert(chars[index], path);
                }
            }

            ncurses::printw("[<] Next Page\n");
            ncurses::printw("[>] Previous Page\n");

            ncurses::refresh();

            match ncurses::getch() as u8 {
                b'<' => {
                    if page > 0 {
                        page -= 1;
                    }
                }
                b'>' => {
                    if page < pages {
                        page += 1;
                    }
                }
                select => {
                    let option = options.get(&select);
                    if option != None {
                        selection = Some(option.unwrap());
                    }
                }
            }

            ncurses::clear();
            ncurses::refresh();
        }

        ncurses::endwin();
        selection.unwrap()
    }
}

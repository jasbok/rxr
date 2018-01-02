extern crate ncurses;

use std::collections::HashSet;
use std::path::PathBuf;

static OPTS: [u8; 40] = [
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'q', b'w', b'e', b'r', b't', b'y',
    b'u', b'i', b'o', b'p', b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'z', b'x',
    b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/',
];

pub struct Menu<'a> {
    paths: &'a [PathBuf],
    items: Vec<&'a str>,
    page: usize,
    pages: usize,
    page_size: usize,
    selected: HashSet<usize>,
}

impl<'a> Menu<'a> {
    pub fn from(paths: &'a [PathBuf]) -> Menu<'a> {
        let items = paths.iter().filter_map(|p| p.as_path().to_str()).collect();
        let page_size = 20;

        Menu::<'a> {
            paths: paths,
            items: items,
            page: 0,
            page_size: page_size,
            pages: paths.len() / page_size,
            selected: HashSet::new(),
        }
    }

    fn get_page_range(&self) -> (usize, usize) {
        let start = self.page * self.page_size;

        let end = if start + self.page_size > self.paths.len() {
            self.paths.len()
        } else {
            start + self.page_size
        };

        (start, end)
    }

    fn display_header(&self) {
        ncurses::printw(&format!(
            "Select Executable (Page {} of {})\n",
            self.page + 1,
            self.pages + 1
        ));
    }

    fn display_page(&self) {
        let (start, end) = self.get_page_range();

        for index in start..end {
            let opt = OPTS[index % self.page_size] as char;
            let item = self.items[index];

            if self.selected.contains(&index) {
                ncurses::printw(&format!("[{}] (x) {}\n", opt, item));
            } else {
                ncurses::printw(&format!("[{}] ( ) {}\n", opt, item));
            }
        }
    }

    fn display_controls(&self) {
        ncurses::printw("[<] Next Page\n");
        ncurses::printw("[>] Previous Page\n");
        ncurses::printw("[Enter] Continue\n");
    }

    fn handle_input(&mut self) -> bool {
        match ncurses::getch() as u8 {
            b'<' => {
                if self.page > 0 {
                    self.page -= 1;
                }
            }
            b'>' => {
                if self.page < self.pages {
                    self.page += 1;
                }
            }
            10 | 13 => {
                return false;
            }
            select => {
                if let Some(opt) = OPTS.iter().position(|&c| c == select) {
                    if self.selected.contains(&opt) {
                        self.selected.remove(&opt);
                    } else {
                        self.selected.insert(opt.clone());
                    }
                } else {

                }
            }
        }

        true
    }

    pub fn get_selected(&self) -> &HashSet<usize> {
        &self.selected
    }

    pub fn display(&mut self) {
        ncurses::initscr();

        loop {
            ncurses::clear();
            self.display_header();
            self.display_page();
            self.display_controls();
            ncurses::refresh();

            if !self.handle_input() {
                break;
            }
        }

        ncurses::endwin();
    }
}

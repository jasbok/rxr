use std::process;

extern crate clap;
extern crate rxr;

fn main() {
    if let Err(e) = rxr::run() {
        println!("[Application error] {}", e);

        process::exit(1);
    }
}

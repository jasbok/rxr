extern crate rxr;

use std::env;
use std::process;

use rxr::args::Args;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rxr_args = Args::new(args.as_slice()).unwrap_or_else(|err| {
        println!("Could not parse args: {}", err);
        process::exit(1);
    });

    if let Err(e) = rxr::run(rxr_args) {
        println!("[Application error] {}", e);

        process::exit(1);
    }
}

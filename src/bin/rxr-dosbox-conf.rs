extern crate clap;
extern crate rxr;

use std::error::Error;
use std::path::PathBuf;
use std::process;

use rxr::dosbox_conf::DosboxConfig;

fn run(args: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let source_path = args.value_of("source").map(PathBuf::from).unwrap();
    let target_path = args.value_of("target").map(PathBuf::from).unwrap();
    let destination_path = args.value_of("destination")
        .map(PathBuf::from)
        .unwrap_or(target_path.clone());

    let source = DosboxConfig::read(&source_path)?;
    let target = DosboxConfig::read(&target_path)?;

    println!("Source: {:#?}", source);
    println!("Target: {:#?}", target);

    let merged = source.merge(&target);

    println!("Merged: {:#?}", merged);

    merged.write(&destination_path)?;

    Ok(())
}

fn main() {
    let command_line = clap::App::new(
        "rxr-dosbox-conf - Rust eXtract and Run Dosbox Config Utility",
    ).version(env!("CARGO_PKG_VERSION"))
        .author("Stefan Alberts <stefan6573@gmail.com>")
        .about("Utility to process and merge dosbox configs.")
        .arg(
            clap::Arg::with_name("source")
                .help("Source config file to be referenced.")
                .required(true)
                .value_name("source")
        )
        .arg(
            clap::Arg::with_name("target")
                .help("Target config file to be processed.")
                .required(true)
                .value_name("target")
        )
        .arg(
            clap::Arg::with_name("destination")
                .help("Destination path to which processed config will be written. Defaults to overwriting target.")
                .required(false)
                .value_name("destination")
        )
        .get_matches();

    if let Err(e) = run(&command_line) {
        println!("[rxr-dosbox-conf error] {}", e);

        process::exit(1);
    }
}

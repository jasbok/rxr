use std::process;

extern crate clap;
extern crate rxr;

fn main() {
    let command_line = clap::App::new("rxr - Rust eXtract and Run")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Stefan Alberts <stefan6573@gmail.com>")
        .about("Extracts and runs archives using configured extractors and executors.")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("config")
                .help("Configuration file to use.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("config_dir")
                .long("config-directory")
                .value_name("configuration directory")
                .help("Configuration directory to use.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("data_dir")
                .short("d")
                .long("data-directory")
                .value_name("data directory")
                .help("Data directory to use, otherwise environment variables RXR_DATA_DIR or XDG_DATA_HOME will be used, lastly falling back to the config file entry (paths.data).")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("target_dir")
                .short("o")
                .long("target-directory")
                .value_name("target directory")
                .help("Target directory to which archive will be extracted, archive will otherwise be extracted to {tempory-directory}/{archive name}")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("temp_dir")
                .short("t")
                .long("temporary-directory")
                .value_name("temp")
                .help("Temp directory to use, otherwise environment variables RXR_TEMP_DIR or XDG_DATA_HOME will be used, lastly falling back to the config file entry (paths.temp).")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("extractor")
                .short("x")
                .long("extractor")
                .value_name("extractor")
                .help("Extractor to use, otherwise the executor that has the highest match based on configured heuristics.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("profile")
                .short("p")
                .long("profile")
                .value_name("profile")
                .help("Profile/Executor to use, otherwise the profile that has the highest match based on configured heuristics.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("archives")
                .help("Archives to be extracted.")
                .required(true)
                .value_name("archives")
                .multiple(true),
        )
        .get_matches();

    if let Err(e) = rxr::run(&command_line) {
        println!("[Application error] {}", e);

        process::exit(1);
    }
}

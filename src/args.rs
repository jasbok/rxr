use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Args {
    pub profile: String,
    pub archive: PathBuf,
    pub config: PathBuf,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 4 {
            return Err("Need to provide at least profile, archive and config arguments.");
        }

        Ok(Args {
            profile: args[1].clone(),
            archive: PathBuf::from(&args[2]),
            config: PathBuf::from(&args[3]),
        })
    }
}

use super::source::Source;

pub struct Compiled {
    pub config: Option<String>,
    pub data_dir: Option<String>,
    pub temp_dir: Option<String>,
}

impl Compiled {
    pub fn new() -> Compiled {
        Compiled {
            config: option_env!("RXR_CONFIG").map(String::from),
            data_dir: option_env!("RXR_DATA_DIR").map(String::from),
            temp_dir: option_env!("RXR_TEMP_DIR").map(String::from),
        }
    }
}

impl From<Compiled> for Source {
    fn from(compiled: Compiled) -> Source {
        Source {
            config: compiled.config,
            data_dir: compiled.data_dir,
            temp_dir: compiled.temp_dir,
            ..Default::default()
        }
    }
}

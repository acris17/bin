use std::env;

pub struct Config {
    pub bin_dir: String,
}
impl Config {
    pub fn new() -> Config {
        Config {
            bin_dir: env::var("BIN_DIR").unwrap_or_default(),
        }
    }
}

pub mod args;
pub mod config;

use crate::cmd;
use args::{Args, Commands};
use config::Config;

pub struct App {
    args: Args,
    config: Config,
}
impl App {
    pub fn new(args: Args, config: Config) -> App {
        App { args, config }
    }
    pub fn run(&self) {
        let command = &self.args.command;
        let bin_dir = &self.config.bin_dir;

        match command {
            Commands::Install { path } => {
                cmd::install(bin_dir, path);
            }
            Commands::Uninstall { name } => {
                cmd::uninstall(bin_dir, name);
            }
            Commands::List => {
                cmd::list(bin_dir);
            }
        }
    }
}

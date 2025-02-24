use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Copy program to the binary directory
    Install {
        /// Path of the program to install
        path: String,
    },
    /// Remove program from the binary directory
    Uninstall {
        /// Name of the program to uninstall
        name: String,
    },
    /// List programs in the binary directory
    List,
}

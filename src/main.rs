use bin::app::{App, args::Args, config::Config};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let config = Config::new();
    let app = App::new(args, config);

    app.run();
}

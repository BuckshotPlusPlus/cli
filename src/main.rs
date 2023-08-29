mod commands;

use clap::Parser;
use commands::template;

#[derive(Parser)]
#[command(name = "bpp", bin_name = "bpp")]
#[command(about = "Revolutionize the web 🚀", long_about = None)]
enum Cli {
    Template,
}

fn main() {
    match Cli::parse() {
        Cli::Template => template::run(),
    }
}

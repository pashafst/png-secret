use clap::Parser;
use png_secret::args::Cli;
use png_secret::Result;
use png_secret::commands::*;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        png_secret::args::Command::Encode(args) => encode_handler(args),
        png_secret::args::Command::Decode(args) => decode_handler(args),
        png_secret::args::Command::Remove(args) => remove_handler(args),
        png_secret::args::Command::Print(args) => print_handler(args),
    }
}


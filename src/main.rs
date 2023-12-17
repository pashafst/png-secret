use clap::Parser;
use pngme::args::Cli;
use pngme::Result;
use pngme::commands::*;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        pngme::args::Command::Encode(args) => encode_handler(args),
        pngme::args::Command::Decode(args) => decode_handler(args),
        pngme::args::Command::Remove(args) => remove_handler(args),
        pngme::args::Command::Print(args) => print_handler(args),
    }
}


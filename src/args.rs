use clap::{Subcommand, Parser, Args};

use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    /// Path to png file
    pub file_path: PathBuf,
    /// 4 char case sensitive chunk type (Can be anything)
    pub chunk_type: String,
    /// Message to embed
    pub message: String,
    /// output file
    pub output_file: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    /// Path to png file
    pub file_path: PathBuf,
    /// 4 char case sensitive chunk type (Can be anything)
    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Path to png file
    pub file_path: PathBuf,
    /// 4 char case sensitive chunk type (Can be anything)
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    /// Path to png file
    pub file_path: PathBuf,
}

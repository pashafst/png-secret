pub mod args;
mod chunk;
mod chunk_type;
pub mod commands;
pub mod png;
mod error;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

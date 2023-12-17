use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum PngError {
    InvalidChunkLength(String),
    InvalidByte(u8),
    InvalidChecksum(u32),
    InvalidHeader,
    ChunkTypeDontExist,
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChunkLength(str) => {
                write!(f, "Invalid string length! The length must be 4: {}", str)
            }
            Self::InvalidByte(b) => write!(f, "Invalid byte!: {}", b),
            Self::InvalidChecksum(crc) => write!(f, "Invalid checksum!: {}", crc),
            Self::InvalidHeader => write!(f, "Invalid header!"),
            Self::ChunkTypeDontExist => write!(f, "Chunk type don't exist!"),
        }
    }
}

impl Error for PngError {}

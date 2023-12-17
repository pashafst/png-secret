use core::fmt;

#[derive(Debug)]
pub enum PngError {
    InvalidChunkLength(String),
    InvalidByte(u8),
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChunkLength(str) => {
                write!(f, "Invalid string length! The length must be 4: {}", str)
            }
            Self::InvalidByte(b) => write!(f, "Invalid byte!: {}", b),
        }
    }
}

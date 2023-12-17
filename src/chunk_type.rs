use crate::error::PngError;
use core::fmt;
use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkType([u8; 4]);

fn byte_is_valid(b: u8) -> bool {
    match b {
        65..=90 | 97..=122 => true,
        _ => false,
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self([value[0], value[1], value[2], value[3]]))
    }
}

impl FromStr for ChunkType {
    type Err = PngError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.bytes();

        // Error checks
        if bytes.len() != 4 {
            return Err(PngError::InvalidChunkLength(s.to_string()));
        }

        bytes.clone().try_for_each(|b| {
            if !byte_is_valid(b) {
                return Err(PngError::InvalidByte(b));
            }
            Ok(())
        })?;

        let vec = bytes.collect::<Vec<_>>();
        let bytes: [u8; 4] = [vec[0], vec[1], vec[2], vec[3]];
        Ok(Self(bytes))
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.0;
    }

    pub fn is_critical(&self) -> bool {
        const ANCILLARY_BIT: u8 = 1u8 << 5;
        let byte = self.0[0];
        byte & ANCILLARY_BIT == 0
    }

    pub fn is_public(&self) -> bool {
        const PRIVATE_BIT: u8 = 1u8 << 5;
        let byte = self.0[1];
        byte & PRIVATE_BIT == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        const RESERVED_BIT: u8 = 1u8 << 5;
        let byte = self.0[2];
        byte & RESERVED_BIT == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        const SAFE_TO_COPY_BIT: u8 = 1u8 << 5;
        let byte = self.0[3];
        byte & SAFE_TO_COPY_BIT != 0
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

use std::convert::TryFrom;
use std::fmt;
use std::str::{from_utf8, FromStr};

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    data_bytes: [u8; 4],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        return self.data_bytes.clone();
    }

    fn is_bytes_valid(&self) -> bool {
        // all bytes should be represented by the characters A-Z or a-z
        for byte in self.data_bytes {
            if !((65 <= byte && byte <= 90) || (97 <= byte && byte <= 122)) {
                return false;
            }
        }
        return true;
    }

    fn is_valid(&self) -> bool {
        return self.is_bytes_valid() && self.is_reserved_bit_valid();
    }

    fn is_critical(&self) -> bool {
        return (self.data_bytes[0] >> 5) & 1 == 0;
    }

    fn is_public(&self) -> bool {
        return (self.data_bytes[1] >> 5) & 1 == 0;
    }

    fn is_reserved_bit_valid(&self) -> bool {
        return (self.data_bytes[2] >> 5) & 1 == 0;
    }

    fn is_safe_to_copy(&self) -> bool {
        return (self.data_bytes[3] >> 5) & 1 == 1;
    }
}

/*
    TryFrom : how to create a object of this type according to another type
    TryInto : how to create a object of another type from this type
*/
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        let chunk_type = ChunkType { data_bytes: bytes };
        if chunk_type.is_bytes_valid() {
            return Ok(ChunkType { data_bytes: bytes });
        }
        return Err(Error::from("Invalid ChunkType byte values\n"));
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 4 {
            let s_bytes = s.as_bytes();
            let chunk_type = ChunkType {
                data_bytes: [s_bytes[0], s_bytes[1], s_bytes[2], s_bytes[3]],
            };
            if chunk_type.is_bytes_valid() {
                return Ok(chunk_type);
            }
            return Err(Error::from("Invalid ChunkType byte values\n"));
        }
        return Err(Error::from("Invalid ChunkType length.\n"));
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", from_utf8(&self.data_bytes).unwrap());
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
        // test the situation that the byte values are correct but the reserved bit is incorrect
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());
        // test the situation that the byte values are incorrect and an Err object should be returned
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

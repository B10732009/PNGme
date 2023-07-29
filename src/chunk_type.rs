use std;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    m_bytes: [u8; 4],
}

impl std::convert::TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        return Ok(Self { m_bytes: bytes });
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "ChunkType: {{ bytes: {:?}}}", &self.m_bytes);
    }
}

impl std::str::FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(String::from("[ChunkType] Invalid ChunkType byte length."));
        }

        let s_bytes = s.as_bytes();
        if !Self::is_bytes_valid(s_bytes) {
            return Err(String::from("[ChunkType] Invalid ChunkType byte values."));
        }

        return Ok(Self {
            m_bytes: s_bytes.try_into().unwrap(),
        });
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.m_bytes.clone();
    }

    pub fn is_valid(&self) -> bool {
        return Self::is_bytes_valid(&self.m_bytes) && self.is_reserved_bit_valid();
    }

    fn is_bytes_valid(bytes: &[u8]) -> bool {
        for b in bytes {
            match b {
                65..=90 | 97..=122 => (),
                _ => return false,
            }
        }
        return true;
    }

    pub fn is_critical(&self) -> bool {
        return (self.m_bytes[0] >> 5) & 1 == 0;
    }

    pub fn is_public(&self) -> bool {
        return (self.m_bytes[1] >> 5) & 1 == 0;
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        return (self.m_bytes[2] >> 5) & 1 == 0;
    }

    pub fn is_safe_to_copy(&self) -> bool {
        return (self.m_bytes[3] >> 5) & 1 == 1;
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

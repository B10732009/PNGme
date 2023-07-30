use std::fmt;
use std::str::from_utf8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    m_bytes: [u8; 4],
}

impl ChunkType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() != 4 {
            return Err(String::from("[ChunkType] Invalid byte length."));
        }

        if !Self::is_bytes_valid(bytes) {
            return Err(String::from("[ChunkType] Invalid byte values."));
        }

        let m_bytes: [u8; 4] = bytes.try_into().unwrap();
        return Ok(Self { m_bytes });
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        return Self::from_bytes(s.as_bytes());
    }

    pub fn bytes(&self) -> &[u8] {
        return &self.m_bytes;
    }

    pub fn str(&self) -> &str {
        return from_utf8(self.bytes()).unwrap();
    }

    pub fn is_valid(&self) -> bool {
        return Self::is_bytes_valid(self.bytes()) && self.is_reserved_bit_valid();
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

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChunkType: {{ Bytes: {:?}}}", self.bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let bytes: [u8; 4] = [82, 117, 83, 116];
        let chunk_type = ChunkType::from_bytes(&[82, 117, 83, 116]).unwrap();
        let ret_bytes: [u8; 4] = chunk_type.bytes().try_into().unwrap();
        assert_eq!(bytes, ret_bytes);
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let s = "RuSt";
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let ret_s = chunk_type.str();
        assert_eq!(s, ret_s);
    }

    #[test]
    pub fn test_chunk_type_bytes_and_str() {
        let chunk_type_1 = ChunkType::from_bytes(&[82, 117, 83, 116]).unwrap();
        let chunk_type_2 = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(chunk_type_1, chunk_type_2);
    }

    #[test]
    pub fn test_chunk_type_bit_attributes() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk_type.is_critical());

        let chunk_type = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk_type.is_critical());

        let chunk_type = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk_type.is_public());

        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk_type.is_public());

        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk_type.is_reserved_bit_valid());

        let chunk_type = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk_type.is_reserved_bit_valid());

        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk_type.is_safe_to_copy());

        let chunk_type = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk_type.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_validation() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk_type.is_valid());

        let chunk_type = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk_type.is_valid());

        let chunk_type_res = ChunkType::from_str("Ru1t");
        assert!(chunk_type_res.is_err());
    }

    #[test]
    pub fn test_chunk_type_display() {
        let chunk_type: ChunkType = ChunkType::from_str("RuSt").unwrap();
        let _ = format!("{}", chunk_type);
    }
}

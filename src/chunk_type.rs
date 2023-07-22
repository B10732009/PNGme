use std;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    m_bytes: [u8; 4],
}

impl ChunkType {
    fn from_bytes(bytes: [u8; 4]) -> Result<ChunkType, String> {
        let chunk_type = ChunkType { m_bytes: bytes };
        if chunk_type.is_bytes_valid() {
            return Ok(chunk_type);
        }
        return Err(String::from("Invalid ChunkType byte values."));
    }

    fn to_bytes(&self) -> [u8; 4] {
        return self.m_bytes.clone();
    }

    fn from_str(s: &str) -> Result<ChunkType, String> {
        if s.len() == 4 {
            let s_bytes = s.as_bytes();
            let chunk_type = ChunkType {
                m_bytes: [s_bytes[0], s_bytes[1], s_bytes[2], s_bytes[3]],
            };
            if chunk_type.is_bytes_valid() {
                return Ok(chunk_type);
            }
            return Err(String::from("Invalid ChunkType byte values."));
        }
        return Err(String::from("Invalid ChunkType length."));
    }

    fn to_str(&self) -> String {
        return String::from(std::str::from_utf8(&self.m_bytes).unwrap());
    }

    fn is_bytes_valid(&self) -> bool {
        // all bytes should be represented by the characters A-Z or a-z
        for byte in self.m_bytes {
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
        return (self.m_bytes[0] >> 5) & 1 == 0;
    }

    fn is_public(&self) -> bool {
        return (self.m_bytes[1] >> 5) & 1 == 0;
    }

    fn is_reserved_bit_valid(&self) -> bool {
        return (self.m_bytes[2] >> 5) & 1 == 0;
    }

    fn is_safe_to_copy(&self) -> bool {
        return (self.m_bytes[3] >> 5) & 1 == 1;
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", std::str::from_utf8(&self.m_bytes).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::from_bytes([82, 117, 83, 116])
            .unwrap()
            .to_bytes();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = "RuSt";
        let actual = ChunkType::from_str("RuSt").unwrap().to_str();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_from_bytes_and_str() {
        let chunk_type_1 = ChunkType::from_bytes([82, 117, 83, 116]).unwrap();
        let chunk_type_2 = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(chunk_type_1, chunk_type_2);
    }

    #[test]
    pub fn test_chunk_type_attributes() {
        // test is_critical()
        let chunk_type_1 = ChunkType::from_str("RuSt").unwrap();
        let chunk_type_2 = ChunkType::from_str("ruSt").unwrap();
        assert!(chunk_type_1.is_critical());
        assert!(!chunk_type_2.is_critical());
        // test is_public()
        let chunk_type_1 = ChunkType::from_str("RUSt").unwrap();
        let chunk_type_2 = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk_type_1.is_public());
        assert!(!chunk_type_2.is_public());
        // test  is_reserved_bit_valid()
        let chunk_type_1 = ChunkType::from_str("RuSt").unwrap();
        let chunk_type_2 = ChunkType::from_str("Rust").unwrap();
        assert!(chunk_type_1.is_reserved_bit_valid());
        assert!(!chunk_type_2.is_reserved_bit_valid());
        // test is_safe_to_copy()
        let chunk_type_1 = ChunkType::from_str("RuSt").unwrap();
        let chunk_type_2 = ChunkType::from_str("RuST").unwrap();
        assert!(chunk_type_1.is_safe_to_copy());
        assert!(!chunk_type_2.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_valid() {
        // test the situation that the object is valid
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk_type.is_valid());
        // test the situation that the byte values are correct but the reserved bit is incorrect
        let chunk_type = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk_type.is_valid());
        // test the situation that the byte values are incorrect and an Err object should be returned
        let chunk_type = ChunkType::from_str("Ru1t");
        assert!(chunk_type.is_err());
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let chunk_type_str = format!("{}", chunk_type);
        assert_eq!(chunk_type_str, "RuSt");
    }
}

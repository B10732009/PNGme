use std;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    m_bytes: [u8; 4],
}

impl ChunkType {
    pub fn from_bytes(bytes: &[u8]) -> Result<ChunkType, String> {
        if bytes.len() == 4 {
            let _chunk_type = ChunkType {
                m_bytes: [bytes[0], bytes[1], bytes[2], bytes[3]],
            };
            if _chunk_type.is_bytes_valid() {
                return Ok(_chunk_type);
            }
            return Err(String::from("Invalid ChunkType byte values."));
        }
        return Err(String::from("Invalid ChunkType byte length."));
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        return self.m_bytes.clone();
    }

    pub fn from_str(s: &str) -> Result<ChunkType, String> {
        return ChunkType::from_bytes(s.as_bytes());
    }

    pub fn to_str(&self) -> String {
        return String::from(std::str::from_utf8(&self.m_bytes).unwrap());
    }

    pub fn is_bytes_valid(&self) -> bool {
        // all bytes should be represented by the characters A-Z or a-z
        for byte in self.m_bytes {
            if !((65 <= byte && byte <= 90) || (97 <= byte && byte <= 122)) {
                return false;
            }
        }
        return true;
    }

    pub fn is_valid(&self) -> bool {
        return self.is_bytes_valid() && self.is_reserved_bit_valid();
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

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let bytes = [82, 117, 83, 116];
        let ret_bytes = ChunkType::from_bytes(&bytes).unwrap().to_bytes().clone();
        assert_eq!(bytes, ret_bytes);
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let s = "RuSt";
        let ret_s = ChunkType::from_str(s).unwrap().to_str();
        assert_eq!(s, ret_s);
    }

    #[test]
    pub fn test_chunk_type_from_bytes_and_str() {
        let ct1 = ChunkType::from_bytes(&[82, 117, 83, 116]).unwrap();
        let ct2 = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(ct1, ct2);
    }

    #[test]
    pub fn test_chunk_type_attributes() {
        // test is_critical()
        let ct1 = ChunkType::from_str("RuSt").unwrap();
        let ct2 = ChunkType::from_str("ruSt").unwrap();
        assert!(ct1.is_critical());
        assert!(!ct2.is_critical());
        // test is_public()
        let ct1 = ChunkType::from_str("RUSt").unwrap();
        let ct2 = ChunkType::from_str("RuSt").unwrap();
        assert!(ct1.is_public());
        assert!(!ct2.is_public());
        // test  is_reserved_bit_valid()
        let ct1 = ChunkType::from_str("RuSt").unwrap();
        let ct2 = ChunkType::from_str("Rust").unwrap();
        assert!(ct1.is_reserved_bit_valid());
        assert!(!ct2.is_reserved_bit_valid());
        // test is_safe_to_copy()
        let ct1 = ChunkType::from_str("RuSt").unwrap();
        let ct2 = ChunkType::from_str("RuST").unwrap();
        assert!(ct1.is_safe_to_copy());
        assert!(!ct2.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_valid() {
        // test the situation that the object is valid
        let ct = ChunkType::from_str("RuSt").unwrap();
        assert!(ct.is_valid());
        // test the situation that the byte values are correct but the reserved bit is incorrect
        let ct = ChunkType::from_str("Rust").unwrap();
        assert!(!ct.is_valid());
        // test the situation that the byte values are incorrect and an Err object should be returned
        let ct = ChunkType::from_str("Ru1t");
        assert!(ct.is_err());
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let ct = ChunkType::from_str("RuSt").unwrap();
        let ct_s = format!("{}", ct);
        assert_eq!(ct_s, "RuSt");
    }
}

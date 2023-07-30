use std::fmt;
use std::str::from_utf8;

use crate::chunk_type::ChunkType;

use crc::crc32::checksum_ieee;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    m_length: u32,
    m_chunk_type: ChunkType,
    m_data: Vec<u8>,
    m_crc: u32,
}

impl Chunk {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 12 {
            return Err(String::from("[Chunk] Invalid byte length."));
        }

        let m_length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());

        let m_chunk_type = ChunkType::from_bytes(&bytes[4..8])?;
        if !m_chunk_type.is_valid() {
            return Err(String::from("[Chunk] Invalid ChunkType."));
        }

        let m_data = bytes[8..(bytes.len() - 4)].to_vec();
        let m_crc = u32::from_be_bytes(bytes[(bytes.len() - 4)..(bytes.len())].try_into().unwrap());

        let real_crc_bytes = &bytes[4..(bytes.len() - 4)];
        let real_crc = checksum_ieee(real_crc_bytes);
        if m_crc != real_crc {
            return Err(String::from("[Chunk] Invalid CRC value."));
        }

        return Ok(Self {
            m_length,
            m_chunk_type,
            m_data,
            m_crc,
        });
    }

    pub fn from_str(chunk_type: &str, data: &str) -> Result<Self, String> {
        let m_length = data.len() as u32;
        let m_chunk_type = ChunkType::from_str(chunk_type)?;
        let m_data = data.as_bytes().to_vec();

        let m_crc_bytes: Vec<u8> = m_chunk_type
            .bytes()
            .iter()
            .chain(m_data.iter())
            .copied()
            .collect();
        let m_crc = checksum_ieee(&m_crc_bytes);

        return Ok(Self {
            m_length,
            m_chunk_type,
            m_data,
            m_crc,
        });
    }

    pub fn length(&self) -> u32 {
        return self.m_length;
    }

    pub fn chunk_type(&self) -> &ChunkType {
        return &self.m_chunk_type;
    }

    pub fn data(&self) -> &[u8] {
        return &self.m_data;
    }

    pub fn data_str(&self) -> &str {
        return from_utf8(self.data()).unwrap();
    }

    pub fn crc(&self) -> u32 {
        return self.m_crc;
    }

    pub fn bytes(&self) -> Vec<u8> {
        let length_bytes = u32::to_be_bytes(self.length());
        let chunk_type_bytes = self.chunk_type().bytes();
        let data_bytes = self.data();
        let crc_bytes = u32::to_be_bytes(self.crc());

        let bytes: Vec<u8> = length_bytes
            .iter()
            .chain(chunk_type_bytes.iter())
            .chain(data_bytes.iter())
            .chain(crc_bytes.iter())
            .copied()
            .collect();

        return bytes;
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chunk: {{ Length: {:?}, {}, ChunkData: {:?}, Crc: {:?}}}",
            self.length(),
            self.chunk_type(),
            self.data(),
            self.crc()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_from_bytes() {
        let length_bytes = u32::to_be_bytes(42);
        let chunk_type_bytes = "RuSt".as_bytes();
        let data_bytes = "This is where your secret message will be!".as_bytes();
        let crc_bytes = u32::to_be_bytes(2882656334);

        let bytes: Vec<u8> = length_bytes
            .iter()
            .chain(chunk_type_bytes.iter())
            .chain(data_bytes.iter())
            .chain(crc_bytes.iter())
            .copied()
            .collect();

        let chunk = Chunk::from_bytes(&bytes).unwrap();

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().str(), "RuSt");
        assert_eq!(
            chunk.data_str(),
            "This is where your secret message will be!"
        );
        assert_eq!(chunk.crc(), 2882656334);
        assert_eq!(chunk.bytes(), bytes);
    }

    #[test]
    pub fn test_chunk_from_str() {
        let chunk_type_str = "RuSt";
        let data_str = "This is where your secret message will be!";

        let chunk = Chunk::from_str(chunk_type_str, data_str).unwrap();

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().str(), "RuSt");
        assert_eq!(
            chunk.data_str(),
            "This is where your secret message will be!"
        );
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    pub fn test_chunk_validation() {
        let length_bytes = u32::to_be_bytes(42);
        let chunk_type_bytes = "RuSt".as_bytes();
        let data_bytes = "This is where your secret message will be!".as_bytes();

        // bad crc
        let crc_bytes = u32::to_be_bytes(2882656333);

        let bytes: Vec<u8> = length_bytes
            .iter()
            .chain(chunk_type_bytes.iter())
            .chain(data_bytes.iter())
            .chain(crc_bytes.iter())
            .copied()
            .collect();

        let chunk_res = Chunk::from_bytes(&bytes);
        assert!(chunk_res.is_err());
    }

    #[test]
    pub fn test_chunk_display() {
        let chunk = Chunk::from_str("RuSt", "This is where your secret message will be!").unwrap();
        let _ = format!("{}", chunk);
    }
}

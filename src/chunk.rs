use std;

use crate::chunk_type::ChunkType;
use crc::crc32::checksum_ieee;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    m_length: u32,
    m_chunk_type: ChunkType,
    m_data: Vec<u8>,
    m_crc: u32,
}

impl std::convert::TryFrom<&[u8]> for Chunk {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err(String::from("[Chunk] Invalid Chunk byte length."));
        }

        let length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());

        let chunk_type: [u8; 4] = bytes[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type)?;
        if !chunk_type.is_valid() {
            return Err(String::from("[Chunk] Invalid ChunkType."));
        }

        let data = bytes[8..(bytes.len() - 4)].to_vec();

        let crc = u32::from_be_bytes(bytes[(bytes.len() - 4)..(bytes.len())].try_into().unwrap());
        let crc_bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let real_crc = checksum_ieee(&crc_bytes);
        if crc != real_crc {
            return Err(String::from("[Chunk] Invalid CRC value."));
        }

        return Ok(Chunk {
            m_length: length,
            m_chunk_type: chunk_type,
            m_data: data,
            m_crc: crc,
        });
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // return write!(
        //     f,
        //     "Chunk [length: {:?}, chunk_type: {:?}, data: {:?}, crc: {:?}]",
        //     self.m_length, self.m_chunk_type, self.m_data, self.m_crc
        // );

        return write!(f, "{}", self.data_as_string().unwrap());
    }
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc_bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        return Chunk {
            m_length: data.len() as u32,
            m_chunk_type: chunk_type,
            m_data: data,
            m_crc: checksum_ieee(&crc_bytes),
        };
    }

    fn length(&self) -> u32 {
        return self.m_length;
    }

    fn chunk_type(&self) -> &ChunkType {
        return &self.m_chunk_type;
    }

    fn data(&self) -> &[u8] {
        return &self.m_data;
    }

    fn crc(&self) -> u32 {
        return self.m_crc;
    }

    fn data_as_string(&self) -> Result<String, String> {
        if let Ok(s) = std::str::from_utf8(&self.m_data) {
            return Ok(String::from(s));
        }
        return Err(String::from("[Chunk] Fail to transform data to string."));
    }

    fn as_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self
            .m_length
            .to_be_bytes()
            .iter()
            .chain(self.m_chunk_type.bytes().iter())
            .chain(self.m_data.iter())
            .chain(self.m_crc.to_be_bytes().iter())
            .copied()
            .collect();
        return bytes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}

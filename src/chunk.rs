use std;

use crate::chunk_type::ChunkType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    m_length: u32,
    m_chunk_type: ChunkType,
    m_chunk_data: Vec<u8>,
    m_crc: u32,
}

impl Chunk {
    pub fn from_bytes(_bytes: &Vec<u8>) -> Result<Chunk, String> {
        if _bytes.len() >= 12 {
            let _length = u32::from_be_bytes([_bytes[0], _bytes[1], _bytes[2], _bytes[3]]);
            let _chunk_type = ChunkType::from_bytes(&[_bytes[4], _bytes[5], _bytes[6], _bytes[7]])?; // if _chunk_type has error, return it here
            let _chunk_data: Vec<u8> = _bytes[8..(_bytes.len() - 4)].iter().copied().collect();
            let _crc = u32::from_be_bytes([
                _bytes[_bytes.len() - 4],
                _bytes[_bytes.len() - 3],
                _bytes[_bytes.len() - 2],
                _bytes[_bytes.len() - 1],
            ]);
            return Ok(Chunk {
                m_length: _length,
                m_chunk_type: _chunk_type,
                m_chunk_data: _chunk_data,
                m_crc: _crc,
            });
        }
        return Err(String::from("Invalid bytes length."));
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let _bytes: Vec<u8> = (u32::to_be_bytes(self.m_length).iter())
            .chain(self.m_chunk_type.to_bytes().iter())
            .chain(self.m_chunk_data.iter())
            .chain(u32::to_be_bytes(self.m_crc).iter())
            .copied()
            .collect();
        return _bytes;
    }

    pub fn from_items(
        _length: u32,
        _chunk_type: &[u8],
        _chunk_data: &Vec<u8>,
        _crc: u32,
    ) -> Result<Chunk, String> {
        return Ok(Chunk {
            m_length: _length,
            m_chunk_type: ChunkType::from_bytes(_chunk_type)?, // if error occured, return here
            m_chunk_data: _chunk_data.clone(),
            m_crc: _crc,
        });
    }

    fn length(&self) -> u32 {
        return self.m_length;
    }

    fn chunk_type(&self) -> &ChunkType {
        return &self.m_chunk_type;
    }

    fn chunk_data(&self) -> &Vec<u8> {
        return &self.m_chunk_data;
    }

    fn chunk_data_str(&self) -> String {
        return String::from_utf8(self.m_chunk_data.clone()).unwrap();
    }

    fn crc(&self) -> u32 {
        return self.m_crc;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_from_bytes() {
        let len_bytes = u32::to_be_bytes(42);
        let ct_bytes = "RuSt".as_bytes();
        let msg_bytes = "This is where your secret message will be!".as_bytes();
        let crc_bytes = u32::to_be_bytes(2882656334);
        let bytes: Vec<u8> = (len_bytes.iter())
            .chain(ct_bytes.iter())
            .chain(msg_bytes.iter())
            .chain(crc_bytes.iter())
            .copied()
            .collect();
        let ret_bytes = Chunk::from_bytes(&bytes).unwrap().to_bytes();
        assert_eq!(bytes, ret_bytes);
    }

    #[test]
    pub fn test_chunk_from_items() {
        let len = 42;
        let ct = "RuSt".as_bytes();
        let msg = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let crc = 2882656334;
        let c = Chunk::from_items(42, ct, &msg, crc).unwrap();
        assert_eq!(c.length(), len);
        assert_eq!(c.chunk_type().to_bytes(), ct);
        assert_eq!(c.chunk_data().clone(), msg); // clone the vector to compare with input
        assert_eq!(c.crc(), crc);
    }
}

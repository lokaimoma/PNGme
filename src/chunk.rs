use crate::chunk_type::ChunkType;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    type_: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.len()
                < Chunk::CHUNK_TYPE_BYTES_LEN + Chunk::CRC_BYTES_LEN + Chunk::DATA_LEN_BYTES_LEN
        {
            return Err("Type and CRC bytes missing".into());
        }

        let (data_len_bytes, value) = value.split_at(Chunk::DATA_LEN_BYTES_LEN);
        let (chunk_type_bytes, value) = value.split_at(Chunk::CHUNK_TYPE_BYTES_LEN);
        let chunk_type_bytes: [u8; 4] = chunk_type_bytes.try_into()?;
        let type_: ChunkType = chunk_type_bytes.try_into()?;
        if !type_.is_valid() {
            return Err("Chunk type not valid".into());
        }
        let length = u32::from_be_bytes(data_len_bytes.try_into()?) as usize;
        let (data_bytes, crc_bytes) = value.split_at(length);
        let data: Vec<u8> = data_bytes.into();
        let expected_crc: u32 = u32::from_be_bytes(crc_bytes.try_into()?);

        let payload = Self {
            length: length.try_into()?,
            type_,
            data,
            crc: expected_crc,
        };

        let crc_ = payload.crc();
        if expected_crc != crc_ {
            return Err(format!("CRC mismatch. Bytes might have been corrupted\nCRC (Expected : Actual) {expected_crc} : {crc_}").into());
        }

        return Ok(payload);
    }
}

impl Chunk {
    pub const CHUNK_TYPE_BYTES_LEN: usize = 4;
    pub const DATA_LEN_BYTES_LEN: usize = 4;
    pub const CRC_BYTES_LEN: usize = 4;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let bytes = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<u8>>();

        let mut payload = Chunk {
            length: bytes.len() as u32,
            data,
            type_: chunk_type,
            crc: 0,
        };
        payload.crc = payload.crc();
        payload
    }

    pub fn crc(&self) -> u32 {
        let checksum_bytes: Vec<u8> = self
            .type_
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();
        let c = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        c.checksum(&checksum_bytes)
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.type_
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_as_string(&self) -> crate::Result<String> {
        Ok(String::from_utf8(self.data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.data
            .len()
            .to_be_bytes()
            .iter()
            .chain(self.type_.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    }
}

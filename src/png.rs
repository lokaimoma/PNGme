use crate::chunk::Chunk;
use std::convert::TryFrom;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Png {
    chunks: Vec<Chunk>,
}

impl TryFrom<&[u8]> for Png {
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < Png::STD_HEADER_LENGTH {
            return Err("Png bytes length less than 8".into());
        }

        let (std_header_bytes, bytes) = bytes.split_at(Png::STD_HEADER_LENGTH);
        let std_header_bytes: [u8; 8] = std_header_bytes.try_into()?;
        if std_header_bytes != Png::STANDAR_HEADER {
            return Err("First 8 bytes doesn't correpond to the PNG spec".into());
        }
        let mut chunk_data_len: [u8; 4] = [0; 4];
        let mut buf_reader = BufReader::new(bytes);
        let mut chunks: Vec<Chunk> = Vec::new();
        while let Ok(()) = buf_reader.read_exact(&mut chunk_data_len) {
            let chunk_bytes_len = Chunk::CHUNK_TYPE_BYTES_LEN
                + Chunk::CRC_BYTES_LEN
                + u32::from_be_bytes(chunk_data_len.into()) as usize;
            let mut chunk_buffer: Vec<u8> = Vec::with_capacity(chunk_bytes_len);
            buf_reader.read_exact(&mut chunk_buffer)?;
            let chunk_bytes: Vec<u8> = chunk_data_len
                .iter()
                .chain(chunk_buffer.iter())
                .copied()
                .collect();
            let chunk = Chunk::try_from(chunk_bytes.as_slice())?;
            chunks.push(chunk);
        }
        Ok(Self { chunks })
    }
}

impl Png {
    pub const STANDAR_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    pub const STD_HEADER_LENGTH: usize = 8;
}

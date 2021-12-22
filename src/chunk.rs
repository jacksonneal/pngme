use crate::{chunk_type::ChunkType, Error, Result};
use std::{
    fmt,
    io::{BufReader, Read},
};

const MAXIMUM_LENGTH: u32 = (1 << 31) - 1;

/// PNG chunk data.
pub struct Chunk {
    /// Length of this chunk data in bytes.
    length: u32,
    /// Chunk type.
    chunk_type: ChunkType,
    /// Chunk data bytes.
    chunk_data: Vec<u8>,
    /// Cyclic redundancy check.
    crc: u32,
}

impl Chunk {
    /// Construct a chunk with the given type and data.
    #[allow(dead_code)]
    pub(crate) fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
        let length: u32 = chunk_data.len() as u32;
        let crc = crc::crc32::checksum_ieee(&[&chunk_type.bytes(), chunk_data.as_slice()].concat());
        Chunk {
            length,
            chunk_type,
            chunk_data,
            crc,
        }
    }

    /// Length of chunk data.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Chunk type.
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Chunk data.
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    /// Cyclic redundancy check.
    fn crc(&self) -> u32 {
        self.crc
    }

    /// Chunk data as string.  `Err` if failed to decode.
    pub(crate) fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone())?)
    }

    /// All chunk content as bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type().bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            self.chunk_type(),
            self.data_as_string()
                .unwrap_or_else(|_| "[data]".to_string()),
        )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8; 4] = Default::default();

        // Read length bytes
        reader.read_exact(&mut buffer)?;
        let length = u32::from_be_bytes(buffer);

        if length > MAXIMUM_LENGTH {
            return Err(format!("Length is too long ({} > 2^31 - 1)", length))?;
        }

        // Read chunk type bytes
        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        // Read chunk data bytes
        let mut chunk_data = vec![0; usize::try_from(length)?];
        reader.read_exact(&mut chunk_data)?;

        if chunk_data.len() != length.try_into()? {
            return Err(format!(
                "Data (len {}) is the wrong length (expected {})",
                chunk_data.len(),
                length
            ))?;
        }

        // Read crc
        let mut crc_buffer: [u8; 4] = Default::default();
        reader.read_exact(&mut crc_buffer)?;
        let crc = u32::from_be_bytes(crc_buffer);

        let expected_crc =
            crc::crc32::checksum_ieee(&[&chunk_type.bytes(), chunk_data.as_slice()].concat());
        if expected_crc != crc {
            return Err(format!(
                "Invalid checksum {}, expected {}",
                crc, expected_crc,
            ))?;
        }

        Ok(Chunk {
            length,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    pub(crate) fn test_chunk_trait_impls() {
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

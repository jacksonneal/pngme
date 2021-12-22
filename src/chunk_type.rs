use crate::{Error, Result};
use std::{fmt, str};

/// 4-byte PNG chunk type code.
/// See section 3.2 [The PNG spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
/// Type codes are restricted to consist of uppercase and lowercase ASCII letters
/// (A-Z and a-z, or 65-90 and 97-122 decimal)
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ChunkType {
    bytes: [u8; 4],
}

#[allow(dead_code)]
impl ChunkType {
    /// Must be ASCII letters (A-Z and a-z, or 65-90 and 97-122 decimal).
    fn is_valid_bytes(bytes: [u8; 4]) -> bool {
        bytes
            .into_iter()
            .all(|b| u8::is_ascii_uppercase(&b) || u8::is_ascii_lowercase(&b))
        // .all(|b| matches!(b, (b'a'..=b'z') | (b'A'..=b'Z')))
    }

    /// All bytes in this chunk type.
    pub(crate) fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Is this chunk type valid.
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    /// This chunk type is critical if ancillary bit is 0.
    /// Ancillary bit is the 5th bit of first byte.
    fn is_critical(&self) -> bool {
        self.bytes[0] & (0b1 << 5) == 0
    }

    /// This chunk is public if the private bit is 0.
    /// Private bit is the 5th bit of second byte.
    fn is_public(&self) -> bool {
        self.bytes[1] & (0b1 << 5) == 0
    }

    /// The reserved bit is the 5th bit of third byte.
    fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] & (0b1 << 5) == 0
    }

    /// This chunk is safe to copy if its copy bit is 1.
    /// Copy bit is the 5th bit of fourth byte.
    fn is_safe_to_copy(&self) -> bool {
        self.bytes[3] & (0b1 << 5) != 0
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.bytes()).unwrap())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        if !ChunkType::is_valid_bytes(bytes) {
            return Err("invalid bytes")?;
        }
        Ok(ChunkType { bytes })
    }
}

impl str::FromStr for ChunkType {
    type Err = Error;

    fn from_str(str: &str) -> Result<ChunkType> {
        if str.len() != 4 {
            return Err("expected 4 bytes")?;
        }
        let bytes: [u8; 4] = str.as_bytes()[..4].try_into()?;
        ChunkType::try_from(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub(crate) fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        println!("{}", actual);
        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub(crate) fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub(crate) fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub(crate) fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub(crate) fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub(crate) fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub(crate) fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub(crate) fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub(crate) fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub(crate) fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub(crate) fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub(crate) fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub(crate) fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        println!("{}", chunk.to_string());
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub(crate) fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

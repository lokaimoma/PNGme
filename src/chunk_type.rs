use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.bytes.into()).unwrap())
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}

impl TryFrom<String> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 4 || value.len() < 4 {
            return Err("Invalid string length".into());
        }

        let mut bytes = [0u8; 4];

        for (idx, b) in value.into_bytes().into_iter().enumerate() {
            if !b.is_ascii_alphabetic() {
                return Err("All characters must be alphabetic".into());
            }
            bytes[idx] = b;
        }

        return Ok(Self { bytes });
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if !value.is_ascii() {
            return Err("Invalid byte: Byte must be in range [65-90] or [97-122]".into());
        }

        return Ok(Self { bytes: value });
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.to_owned()
    }

    pub fn is_valid(&self) -> bool {
        for b in self.bytes {
            if !b.is_ascii() {
                return false;
            }
        }
        return self.is_reserved_bit_valid();
    }

    pub fn is_critical(&self) -> bool {
        let fifth_bit = (self.bytes[0] >> 5u8) & 1u8;

        if fifth_bit == 0 {
            return true;
        }
        return false;
    }

    pub fn is_public(&self) -> bool {
        let fifth_bit = (self.bytes[1] >> 5u8) & 1u8;

        if fifth_bit == 0 {
            return true;
        }
        return false;
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let fifth_bit = (self.bytes[2] >> 5u8) & 1u8;

        if fifth_bit == 0 {
            return true;
        }
        return false;
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let fifth_bit = (self.bytes[3] >> 5u8) & 1u8;

        if fifth_bit == 1 {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

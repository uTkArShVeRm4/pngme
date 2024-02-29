use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkTypeError;

impl std::fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid ChunkType")
    }
}

impl std::error::Error for ChunkTypeError {}

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
    is_valid: bool,
    is_critical: bool,
    is_public: bool,
    is_reserved_bit_valid: bool,
    is_safe_to_copy: bool,
}
#[allow(dead_code)]
impl ChunkType {
    fn new(bytes: [u8; 4]) -> Result<Self, ChunkTypeError> {
        let is_valid = bytes.iter().all(|&x| x.is_ascii_alphabetic());
        let is_critical = bytes[0].is_ascii_uppercase();
        let is_public = bytes[1].is_ascii_uppercase();
        let is_reserved_bit_valid = bytes[2].is_ascii_uppercase();
        let is_safe_to_copy = bytes[3].is_ascii_lowercase();

        if is_valid {
            Ok(ChunkType {
                bytes,
                is_valid,
                is_critical,
                is_public,
                is_reserved_bit_valid,
                is_safe_to_copy,
            })
        } else {
            Err(ChunkTypeError)
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn is_critical(&self) -> bool {
        self.is_critical
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.is_reserved_bit_valid
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.is_safe_to_copy
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bytes_str = std::str::from_utf8(&self.bytes).unwrap_or("Invalid UTF-8");

        write!(f, "{}", bytes_str)
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let bytes = value.clone();

        ChunkType::new(bytes)
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() >= 4 {
            let mut chunk_bytes = [0; 4];
            chunk_bytes.copy_from_slice(&bytes[..4]);
            ChunkType::new(chunk_bytes)
        } else {
            Err(ChunkTypeError)
        }
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
        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
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

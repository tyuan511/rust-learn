use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;
use std::{fmt, str};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    critical_chunk: u8,
    public_chunk: u8,
    uppercase_always_chunk: u8,
    safe_chunk: u8,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.critical_chunk,
            self.public_chunk,
            self.uppercase_always_chunk,
            self.safe_chunk,
        ]
    }

    fn is_critical(self: &Self) -> bool {
        is_upper_case_letter(&self.critical_chunk)
    }
    fn is_public(&self) -> bool {
        is_upper_case_letter(&self.public_chunk)
    }
    fn is_reserved_bit_valid(&self) -> bool {
        is_upper_case_letter(&self.uppercase_always_chunk)
    }
    fn is_safe_to_copy(&self) -> bool {
        is_lower_case_letter(&self.safe_chunk)
    }
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(arr: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in arr.iter() {
            if !is_lower_case_letter(byte) && !is_upper_case_letter(byte) {
                println!("{:?}", byte);
                return Err("byte is not valid");
            }
        }

        Ok(ChunkType {
            critical_chunk: arr[0],
            public_chunk: arr[1],
            uppercase_always_chunk: arr[2],
            safe_chunk: arr[3],
        })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr = s.as_bytes();
        for byte in arr.iter() {
            if !is_lower_case_letter(byte) && !is_upper_case_letter(byte) {
                return Err("byte is not valid");
            }
        }

        Ok(ChunkType {
            critical_chunk: arr[0],
            public_chunk: arr[1],
            uppercase_always_chunk: arr[2],
            safe_chunk: arr[3],
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = str::from_utf8(&[
            self.critical_chunk,
            self.public_chunk,
            self.uppercase_always_chunk,
            self.safe_chunk,
        ])
        .unwrap()
        .to_string();
        write!(f, "{}", string)
    }
}

fn is_upper_case_letter(byte: &u8) -> bool {
    (&65..&90).contains(&byte)
}

fn is_lower_case_letter(byte: &u8) -> bool {
    (&97..&122).contains(&byte)
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
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

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

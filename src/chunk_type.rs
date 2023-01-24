use std::str;
use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Debug)]
pub struct ChunkType([u8; 4]);

//impliment try from
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for i in value.iter() {
            if i < &0 || i > &255 {
                return Err("INVALID BYTES! GREATER THAN BYTE SIZE".into());
            }
        }

        Ok(ChunkType(value))
    }
}

//impliment from str
impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if s.len() > 4 {
            return Err("STRING IS TOO LONG MUST BE 4 BYTE OF SIZE".into());
        }
        let valid_big= (65..=90).collect::<Vec<u8>>();
        let valid_small = (97..=122).collect::<Vec<u8>>();
        for i in bytes.iter(){
            if !valid_big.contains(i) && !valid_small.contains(i){
                    print!("{}",i);
                    return Err("INVALID INPUT CHUNK".into())
            }
        }
        
        Ok(ChunkType(bytes[..4].try_into().unwrap()))
    }
}

//impliment display
impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0;
        let s = String::from_utf8(vec![s[0], s[1], s[2], s[3]]).unwrap();
        write!(f, "{}", s)
    }
}

// self implimentation
impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.0
    }

    fn is_valid(&self) -> bool {
        
        let the_byte = self.0[2];
        if the_byte < 91 && the_byte > 64 {
            true
        } else {
            false
        }
    }

    fn is_critical(&self) -> bool {
        let the_byte = self.0[0];
        if the_byte < 91 && the_byte > 64 {
            true
        } else {
            false
        }
    }

    fn is_public(&self) -> bool {
        let the_byte = self.0[1];
        if the_byte < 91 && the_byte > 64 {
            true
        } else {
            false
        }
    }

    fn is_reserved_bit_valid(&self) -> bool {
        let the_byte = self.0[2];
        if the_byte < 91 && the_byte > 64 {
            true
        } else {
            false
        }
    }

    fn is_safe_to_copy(&self) -> bool {
        let the_byte = self.0[3];
        if the_byte < 91 && the_byte > 64 {
            false
        } else {
            true
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

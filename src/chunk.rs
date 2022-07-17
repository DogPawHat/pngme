use crate::chunk_type::{ChunkType, ChunkTypeError};
use crc::{Crc, CRC_32_ISO_HDLC};
use std::{array::TryFromSliceError, fmt, string::FromUtf8Error};

#[derive(Debug)]
enum ChunkError {
    NotEnoughBytesCRC(TryFromSliceError),
    NotEnoughBytesLength(TryFromSliceError),
    NotEnoughBytesType(TryFromSliceError),
    NotEnoughBytesData,
    InvalidStringData(FromUtf8Error),
    InvalidCRC,
    Type(ChunkTypeError),
}

#[derive(Debug, Clone)]
pub(crate) struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl From<ChunkTypeError> for ChunkError {
    fn from(e: ChunkTypeError) -> Self {
        Self::Type(e)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let length_field: [u8; 4] = value[0..4]
            .try_into()
            .map_err(ChunkError::NotEnoughBytesLength)?;
        let chunk_type_field: [u8; 4] = value[4..8]
            .try_into()
            .map_err(ChunkError::NotEnoughBytesType)?;

        let length = u32::from_be_bytes(length_field);
        let chunk_type = ChunkType::try_from(chunk_type_field)?;

        let data = value[8..(length + 8) as usize].to_vec();
        if data.len() != length as usize {
            return Err(ChunkError::NotEnoughBytesData);
        }

        let crc_field = value[(length + 8) as usize..]
            .try_into()
            .map_err(ChunkError::NotEnoughBytesCRC)?;
        let crc = u32::from_be_bytes(crc_field);

        let combined_collection: Vec<u8> = chunk_type_field
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        if crc != Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&combined_collection) {
            return Err(ChunkError::InvalidCRC);
        }

        Ok(Chunk {
            chunk_type,
            data,
            crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl Chunk {
    pub(crate) fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let combined_collection: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<_>>();

        Chunk {
            chunk_type,
            data,
            crc: Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(combined_collection.as_ref()),
        }
    }

    pub(crate) fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub(crate) fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.data
    }

    pub(crate) fn crc(&self) -> u32 {
        self.crc
    }

    pub(crate) fn data_as_string(&self) -> Result<String, ChunkError> {
        String::from_utf8(self.data.clone()).map_err(ChunkError::InvalidStringData)
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let length: u32 = self.data.len() as u32;
        length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
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

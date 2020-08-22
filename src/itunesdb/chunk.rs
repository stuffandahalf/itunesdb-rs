use crate::itunesdb::Error;

pub trait Chunk {
    fn field_identifier() -> [u8; 4];
    fn from_bin(buffer: &mut Vec<u8>) -> Result<Box<Self>, Error>;
    fn to_bin(&self) -> Vec<u8>;
    fn header_size(&self) -> u32;
}

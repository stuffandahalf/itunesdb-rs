use crate::itunesdb::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct AlbumItem {
    hdr_size: u32,
    data_count: u32,
    data_objs: Vec<DataObject>,
    album_id: u16
}

impl AlbumItem {
    pub fn new() -> AlbumItem {
        AlbumItem {
            hdr_size: 0,
            data_count: 0,
            data_objs: Vec::<DataObject>::new(),
            album_id: 0
        }
    }
}

impl Chunk for AlbumItem {
    fn field_identifier() -> [u8; 4] {
        *b"mhia"
    }
    
    fn from_bin(buffer: &mut Vec<u8>) -> Result<Box<AlbumItem>, Error> {
        let mut ai = AlbumItem::new();
        
        println!("{:X?}", &buffer[0..4]);
        
        for i in 0..4 {
            if buffer[i] != AlbumItem::field_identifier()[i] {
                return Err(Error::mismatching_field());
            }
        }
        
        ai.hdr_size = u32::from_le_bytes(buffer[4..8]
            .try_into()
            .unwrap());
        ai.data_count = u32::from_le_bytes(buffer[12..16]
            .try_into()
            .unwrap());
        ai.album_id = u16::from_le_bytes(buffer[18..20]
            .try_into()
            .unwrap());
        
        buffer.drain(0..(ai.hdr_size as usize));
        
        Ok(Box::new(ai))
    }
    
    fn to_bin(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        
        buffer.extend(AlbumItem::field_identifier().iter());
        
        buffer
    }
    
    fn header_size(&self) -> u32 {
        if self.hdr_size != 0 {
            self.hdr_size
        } else {
            0x58
        }
    }
}

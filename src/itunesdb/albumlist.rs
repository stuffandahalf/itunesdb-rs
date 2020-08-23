use crate::itunesdb::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct AlbumList {
    hdr_size: u32,
    album_item_count: u32,
    album_items: Vec<AlbumItem>
}

impl AlbumList {
    pub fn new() -> AlbumList {
        AlbumList {
            hdr_size: 12,
            album_item_count: 0,
            album_items: Vec::<AlbumItem>::new()
        }
    }
}

impl Chunk for AlbumList {
    fn field_identifier() -> [u8; 4] {
        *b"mhla"
    }
    
    fn from_bin(buffer: &mut Vec<u8>) -> Result<Box<AlbumList>, Error> {
        #[cfg(debug_assertions)]
        let _ = dbg!(std::str::from_utf8(&buffer[0..4]));
        
        let mut al = AlbumList::new();
        
        for i in 0..4 {
            if buffer[i] != AlbumList::field_identifier()[i] {
                return Err(Error::mismatching_field());
            }
        }
        
        al.hdr_size = u32::from_le_bytes(buffer[4..8]
            .try_into()
            .unwrap());
        al.album_item_count = u32::from_le_bytes(buffer[8..12]
            .try_into()
            .unwrap());
        
        buffer.drain(0..(al.hdr_size as usize));
        for _ in 0..al.album_item_count {
            al.album_items.push(*AlbumItem::from_bin(buffer)?);
        }
        
        Ok(Box::new(al))
    }
    
    fn to_bin(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        
        buffer.extend(AlbumList::field_identifier().iter());
        
        buffer
    }
    
    fn header_size(&self) -> u32 {
        if self.hdr_size != 0 {
            self.hdr_size
        } else {
            12
        }
    }
}

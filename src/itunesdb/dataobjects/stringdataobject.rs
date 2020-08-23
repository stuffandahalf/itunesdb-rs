use crate::itunesdb::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct StringDataObject {
    str_len: u32,
    char_vec: Vec<u8>
}

impl StringDataObject {
    pub fn new() -> StringDataObject {
        StringDataObject {
            str_len: 0,
            char_vec: Vec::<u8>::new()
        }
    }
}

impl Chunk for StringDataObject {
    fn field_identifier() -> [u8; 4] {
        *b"mhod"
    }
    
    fn from_bin(buffer: &mut Vec<u8>) -> Result<Box<StringDataObject>, Error> {
        #[cfg(debug_assertions)]
        let _ = dbg!(std::str::from_utf8(&buffer[0..4]));
        
        let mut strdobj = StringDataObject::new();
        
        for i in 0..4 {
            if buffer[i] != StringDataObject::field_identifier()[i] {
                return Err(Error::mismatching_field());
            }
        }
        
        let obj_size = u32::from_le_bytes(buffer[8..12]
            .try_into()
            .unwrap());
            
        strdobj.str_len = obj_size - strdobj.header_size();
        strdobj.char_vec.extend(buffer[40..((40 + strdobj.str_len) as usize)].iter());
        
        Ok(Box::new(strdobj))
    }
    
    fn to_bin(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        
        buffer.extend(StringDataObject::field_identifier().iter());
        
        buffer
    }
    
    fn header_size(&self) -> u32 {
        0x18
    }
}

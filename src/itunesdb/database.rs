//mod super::itunesdb;
//mod itunesdb;

use std::convert::TryInto;
use crate::itunesdb::*;

#[derive(Debug)]
pub struct DataBase {
	pub hdr_size: u32,
	pub version: u32,
	pub id: u64,
	pub lang: [u8; 2],
	pub lib_persistent_id: u64,
	pub obscure_hash: [u8; 20],
	pub track_list: Option<TrackList>,
	pub playlist_list: Option<PlaylistList>,
	pub podcast_list: Option<PodcastList>,
	pub album_list: Option<AlbumList>,
	pub new_playlist_list: Option<NewPlaylistList>
}

impl DataBase {
	pub fn new() -> DataBase {
		DataBase {
			hdr_size: 0,
			version: 0x17,
			id: 0,
			lang: *b"en",
			lib_persistent_id: 0,
			obscure_hash: [0; 20],
			track_list: None,
			playlist_list: None,
			podcast_list: None,
			album_list: None,
			new_playlist_list: None
		}
	}
}

impl Chunk for DataBase {
	fn field_identifier() -> [u8; 4] {
		*b"mhbd"
	}
	
	fn from_bin(buffer: &Vec<u8>) -> Result<Box<DataBase>, Error> {
		let mut db = DataBase::new();
		
		for i in 0..4 {
			if buffer[i] != DataBase::field_identifier()[i] {
				return Err(Error::mismatching_field());
			}
		}
		
		db.hdr_size = u32::from_le_bytes(buffer[4..8]
			.try_into()
			.unwrap());
		
		db.version = u32::from_le_bytes(buffer[16..20]
			.try_into()
			.unwrap());
		db.id = u64::from_le_bytes(buffer[24..32].try_into().unwrap());
		if db.version >= 0x13 {
			db.lang[0] = buffer[70];
			db.lang[1] = buffer[71];
		}
		if db.version >= 0x14 {
			db.lib_persistent_id = u64::from_le_bytes(buffer[72..80]
				.try_into()
				.unwrap());
		}
		if db.version >= 0x19 {
			db.obscure_hash = buffer[88..108].try_into().unwrap();
		}
		
		Ok(Box::new(db))
	}
	
	fn to_bin(&self) -> Vec<u8> {
		let zero: u32 = 0;
		let one: u32 = 1;
		let two: u32 = 2;
		let mut buffer = Vec::<u8>::new();
		
		// add identifier mhbd
		buffer.extend(DataBase::field_identifier().iter());
		// add size of header
		buffer.extend(self.header_size().to_le_bytes().iter());
		
		// push size of child elements
		// REPLACE THIS
		buffer.extend(self.header_size().to_le_bytes().iter());
		
		// constant u32 1
		buffer.extend(one.to_le_bytes().iter());
		
		// version number
		buffer.extend(self.version.to_le_bytes().iter());
		
		// # of children
		buffer.extend(zero.to_le_bytes().iter());
		
		// db id
		buffer.extend(self.id.to_le_bytes().iter());
		
		// constant u32 2
		buffer.extend(two.to_le_bytes().iter());
		
		if self.version >= 0x11 {
			// unknown u64 field
			buffer.extend(zero.to_le_bytes().iter());
			buffer.extend(zero.to_le_bytes().iter());
		}
		
		buffer
	}
	
	fn header_size(&self) -> u32 {
		let mut sz: u32 = 0;
		
		if self.hdr_size != 9 {
			sz = self.hdr_size;
		}
		else if self.version >= 0x17 {
			sz = 0xBC;
		}
		else {
			sz = 0x68;
		}
		
		sz
	}
}

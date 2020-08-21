//mod super::itunesdb;
//mod itunesdb;

use std::convert::TryInto;
use crate::itunesdb::*;

fn test(buff: &[u8]) -> [u8; 4] {
	buff.try_into().expect("slice with incorrect length")
}

#[derive(Debug)]
pub struct DataBase {
	pub version: u32,
	pub id: u64,
	pub lang: [u8; 2],
	pub track_list: Option<TrackList>,
	pub playlist_list: Option<PlaylistList>,
	pub podcast_list: Option<PodcastList>,
	pub album_list: Option<AlbumList>,
	pub new_playlist_list: Option<NewPlaylistList>
}

impl DataBase {
	pub fn new() -> DataBase {
		DataBase {
			version: 0,
			id: 0,
			lang: ['e' as u8, 'n' as u8],
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
		['m' as u8, 'h' as u8, 'b' as u8, 'd' as u8]
	}
	
	fn from_bin(buffer: Vec<u8>) -> Result<Box<DataBase>, Error> {
		let mut db = DataBase::new();
		
		for i in 0..4 {
			if buffer[i] != DataBase::field_identifier()[i] {
				return Err(Error::mismatching_field());
			}
		}
		
		db.version = u32::from_le_bytes(buffer[16..20].try_into().unwrap());
		db.id = u64::from_le_bytes(buffer[24..32].try_into().unwrap());
		//db.lang = u16::from_le_bytes(buffer[70..72].try_into().unwrap());
		db.lang[0] = buffer[70];
		db.lang[1] = buffer[71];
		
		Ok(Box::new(db))
	}
	
	fn to_bin(&self) -> Vec<u8> {
		let mut buffer = Vec::<u8>::new();
		for byte in DataBase::field_identifier().iter() {
			buffer.push(*byte);
		}
		
		buffer
	}
}

use crate::itunesdb::*;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub enum DataSetType {
    TrackList = 1,
    PlaylistList = 2,
    PodcastList = 3,
    AlbumList = 4,
    NewPlaylistList = 5
}

#[derive(Debug)]
pub struct DataSet {
    pub hdr_size: u32,
    pub data_type: u32,
    
    pub track_list: Option<TrackList>,
    pub playlist_list: Option<PlaylistList>,
    pub podcast_list: Option<PodcastList>,
    pub album_list: Option<AlbumList>,
    pub new_playlist_list: Option<NewPlaylistList>
}

impl DataSet {
    pub fn new() -> DataSet {
        DataSet {
            hdr_size: 0,
            data_type: 0,
            track_list: None,
            playlist_list: None,
            podcast_list: None,
            album_list: None,
            new_playlist_list: None
        }
    }
}

impl Chunk for DataSet {
    fn field_identifier() -> [u8; 4] {
        *b"mhsd"
    }
    
    fn from_bin(buffer: &mut Vec<u8>) -> Result<Box<DataSet>, Error> {
        #[cfg(debug_assertions)]
        let _ = dbg!(std::str::from_utf8(&buffer[0..4]));
        
        let mut ds = DataSet::new();
        
        for i in 0..4 {
            if buffer[i] != DataSet::field_identifier()[i] {
                return Err(Error::mismatching_field());
            }
        }
        
        ds.hdr_size = u32::from_le_bytes(buffer[4..8]
            .try_into()
            .unwrap());
        
        ds.data_type = u32::from_le_bytes(buffer[12..16]
            .try_into()
            .unwrap());
        
        buffer.drain(0..(ds.header_size() as usize));
        match ds.data_type {
            /*_ if ds.data_type == DataSetType::TrackList as u32 =>
                ds.track_list = Some(*TrackList::from_bin(buffer)?),*/
            /*_ if ds.data_type == DataSetType::PlaylistList as u32 =>
                ds.playlist_list = Some(*PlaylistList::from_bin(buffer)?),*/
            /*_ if ds.data_type == DataSetType::PodcastList as u32 =>
                ds.podcast_list = Some(*PodcastList::from_bin(buffer)?),*/
            _ if ds.data_type == DataSetType::AlbumList as u32 =>
                ds.album_list = Some(*AlbumList::from_bin(buffer)?),
            /*_ if ds.data_Type == DataSetType::NewPlaylistList as u32 =>
                ds.new_playlist_list = Some(*NewPlaylistList::from_bin(buffer)?),*/
            _ => return Err(Error::unknown_data_type())
        };
        
        
        Ok(Box::new(ds))
    }
    
    fn to_bin(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        
        buffer.extend(DataSet::field_identifier().iter());
        
        buffer
    }
    
    fn header_size(&self) -> u32 {
        if self.hdr_size > 0 {
            self.hdr_size
        } else {
            0x60
        }
    }
}

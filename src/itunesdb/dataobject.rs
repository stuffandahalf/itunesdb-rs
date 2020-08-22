use crate::itunesdb::*;
use crate::itunesdb::dataobjects::*;
use std::convert::TryInto;

enum DataObjectType {
    Unset = 0,
    Title = 1,
    Location = 2,
    Album = 3,
    Artist = 4,
    Genre = 5,
    Filetype = 6,
    EQSetting = 7,
    Comment = 8,
    Category = 9,
    Composer = 12,
    Grouping = 13,
    DescriptionTest = 14,
    PodcastEnclosureURL = 15,
    PodcastRSSURL = 16,
    ChapterData = 17,
    Subtitle = 18,
    Show = 19,
    EpisodeNumber = 20,
    TVNetwork = 21,
    AlbumArtist = 22,
    ArtistName = 23,
    TrackKeywords = 24,
    TVLocale = 25,
    SortTitle = 27,
    SortAlbum = 28,
    SortAlbumArtist = 29,
    SortComposer = 30,
    SortTVShow = 31,
    UnknownBinary = 32,
    SmartPlaylistData = 50,
    SmartPlaylistRules = 51,
    LibraryPlaylistIndex = 52,
    UnknownSimilarToLibraryPlaylistIndex = 53,
    ColumnSizingOrPlaylistOrderIndicator = 100,
    AlbumListAlbum = 200,
    AlbumListArtist = 201,
    AlbumListSortArtist = 202,
    AlbumListPodcastURL = 203,
    AlbumListTVShow = 204
}

#[derive(Debug)]
pub struct DataObject {
    data_type: u32,
    string_data: Option<StringDataObject>
}

impl DataObject {
    pub fn new() -> DataObject {
        DataObject {
            data_type: 0,
            string_data: None
        }
    }
}

impl Chunk for DataObject {
    fn field_identifier() -> [u8; 4] {
        *b"mhod"
    }
    
    fn from_bin(buffer: &mut Vec<u8>) -> Result<Box<DataObject>, Error> {
        let mut dobj = DataObject::new();
        
        for i in 0..4 {
            if buffer[i] != DataObject::field_identifier()[i] {
                return Err(Error::mismatching_field());
            }
        }
        
        dobj.data_type = u32::from_le_bytes(buffer[12..16]
            .try_into()
            .unwrap());
        
        //buffer.drain(0..(dobj.header_size() as usize));
        match dobj.data_type {
            1..=14|18..=31 => dobj.string_data = Some(*StringDataObject::from_bin(buffer)?),
            //15|16 => dobj.podcast_url_data = Some(*PodcastURLDataObject::from_bin(buffer)?),
            //17 => dobj.chapter_Data = Some(*ChapterDataObject::from_bin(buffer)?),
            // Add more stuff here
            _ => return Err(Error::unknown_data_type())
        };
        
        
        Ok(Box::new(dobj))
    }
    
    fn to_bin(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        
        buffer.extend(DataObject::field_identifier().iter());
        
        buffer
    }
    
    fn header_size(&self) -> u32 {
        match self.data_type {
            1..=14|18..=31 => self.string_data.as_ref().unwrap().header_size(),
            //15|16 => self.podcast_url_data.as_ref().unwrap().header_size(),
            //17 => self.chapter_data.as_ref().unwrap().header_size(),
            _ => 0
        }
    }
}

mod itunesdb;

#[cfg(test)]
mod tests {
	use std::io::Read;
	use crate::itunesdb::{Chunk, DataBase};
	
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
	
	#[test]
	fn identifier_test() {
		assert_eq!(DataBase::field_identifier(), *b"mhbd");
		let db = DataBase::new();
	}
	
	#[test]
	fn db_to_bin_test() {
		println!("{:X?}", DataBase::new().to_bin());
	}
	
	#[test]
	fn bin_to_db_test() {
		let f = std::fs::File::open("/run/media/ganorton/WD 4TB/iPod Video/iPod_Control/iTunes/iTunesDB");
		let mut f = match f {
			Ok(file) => file,
			Err(_) => {
				assert!(false);
				return;
			}
		};
		let mut data = Vec::<u8>::new();
		let l = f.read_to_end(&mut data);
		let mut l = match l {
			Ok(length) => length,
			Err(_) => {
				assert!(false);
				return;
			}
		};
		
		let db = DataBase::from_bin(&data);
		let mut db = match db {
			Ok(db) => db,
			Err(_) => {
				assert!(false);
				return;
			}
		};
		println!("{:X?}", db);
	}
	
	#[test]
	fn bin_to_db_to_bin_test() {
		let f = std::fs::File::open("/run/media/ganorton/WD 4TB/iPod Video/iPod_Control/iTunes/iTunesDB");
		let mut f = match f {
			Ok(file) => file,
			Err(_) => {
				assert!(false);
				return;
			}
		};
		let mut data = Vec::<u8>::new();
		let l = f.read_to_end(&mut data);
		let mut l = match l {
			Ok(length) => length,
			Err(_) => {
				assert!(false);
				return;
			}
		};
		
		let db = DataBase::from_bin(&data);
		let mut db = match db {
			Ok(db) => db,
			Err(_) => {
				assert!(false);
				return;
			}
		};
		
		assert_eq!(db.to_bin(), data);
	}
}

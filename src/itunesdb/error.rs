//use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Error {
	msg: String
}

impl Error {
	pub fn mismatching_field() -> Error {
		Error { msg: String::from("Field mismatch") }
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.msg)
	}
}

impl std::error::Error for Error { }

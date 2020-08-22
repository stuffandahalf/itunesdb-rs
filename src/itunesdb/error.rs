//use std::error::Error;
use std::fmt;
use backtrace::Backtrace;

#[derive(Debug)]
pub struct Error {
    msg: String,
    bt: Backtrace
}

impl Error {
    pub fn message(&self) -> &str {
        &self.msg
    }
    
    pub fn backtrace(&self) -> &Backtrace {
        &self.bt
    }
    
    pub fn mismatching_field() -> Error {
        Error {
            msg: String::from("Field mismatch"),
            bt: Backtrace::new()
        }
    }
    
    pub fn unknown_data_type() -> Error {
        Error {
            msg: String::from("Unrecognized data type"),
            bt: Backtrace::new()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)        
    }
}

impl std::error::Error for Error { }

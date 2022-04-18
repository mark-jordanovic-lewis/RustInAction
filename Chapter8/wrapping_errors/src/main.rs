use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Error;
use std::net;
use std::net::Ipv6Addr;
use std::{error, io};

// Define an enum to wrap the possible errors in your code
// - derive debug for display
#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

// Define display in terms of Debug
impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// implement error::Error (all std methods implemented as above)
impl error::Error for UpstreamError {}

// Define From methods to convert to custom err enum
impl From<io::Error> for UpstreamError {
    fn from(err: io::Error) -> Self {
        UpstreamError::IO(err)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(err: net::AddrParseError) -> Self {
        UpstreamError::Parsing(err)
    }
}

// `?` now performs automatic conversion to UpstreamError enum
fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}

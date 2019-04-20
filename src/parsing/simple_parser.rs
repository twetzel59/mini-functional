//! Implements a simple parser.

use std::{
    io,
    fs::File,
    path::Path,
};

/// An object that owns an open file as it is parsed.
#[derive(Debug)]
pub struct Parser {
    file: File,
}

impl Parser {
    /// Create a new parser with the given open file.
    pub fn new(file: File) -> Self {
        Parser {
            file,
        }
    }

    /// Attempt to create a parser by opening the file at
    /// the supplied path.
    pub fn with_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        Ok(Self::new(file))
    }

    /*pub fn test() -> Result<(), > {

    }*/
}

//! File - Building simple File struct for practice
#![allow(unused_variables, dead_code)]

extern crate core;

use core::fmt;
use std::fmt::Display;


/// File state representation
/// - Open, or,
/// - Closed
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED")
        }
    }
}

/// File representation
/// - probably lives on some file system but who knows?
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>, // data remains private
    pub state: FileState,
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, ({})>", self.name, self.state)
    }
}

impl File {
    /// New file
    /// - Requires filename
    /// - Assumed empty on `new`
    /// # Examples
    /// ```
    /// let f = File::new("filename.txt");
    /// ```
    pub fn new(filename: &str) -> Self {
        File {
            name: String::from(filename),
            data: vec![],
            state: FileState::Closed,
        }
    }

    /// Returns file size in bytes (usize).
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns safe to use (no borrow) filename.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Instantiates a new File populated with data, ready to write.
    pub fn new_with_data(filename: &str, data: &Vec<u8>) -> Self {
        let mut f = File::new(filename);
        f.data = data.clone();
        f
    }

    /// Sets state of File to open
    pub fn open(mut self) -> Result<Self, String> {
        self.state = FileState::Open;
        Ok(self)
    }

    /// Sets state of File to closed
    pub fn close(mut self) -> Result<Self, String> {
        self.state = FileState::Closed;
        Ok(self)
    }

    /// Reads in memory data of File
    /// - returns Ok(text) if Open
    /// - returns Err(text) if Closed
    pub fn read(&mut self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err("File must be open for reading".into())
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length); // nice method to allocate memory
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn main() {
    let mut file = File::new_with_data("my_file.txt", &vec![114, 117, 115, 116, 33]);
    let mut buffer: Vec<u8> = vec![];

    if let Err(error) = file.read(&mut buffer) {
        println!("Error checking is working: {}", error);
    }

    file = file.open().unwrap();
    let file_length = file.read(&mut buffer).unwrap();
    file = file.close().unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("File Debug: {:?}", &file);
    println!("File Display: {}", &file);
    println!("{} is {} bytes long.", &file.name, file_length);
    println!("Content: {}", text);
}

// pointless function for type conversion - still need to unwrap option.
fn optionify_error<T>(maybe: Result<T, String>) -> (Option<T>, String) {
    match maybe {
        Ok(definitely) => (Some(definitely), "".into()),
        Err(response) => (None, response)
    }
}

// maybe more helpful for error messaging, but no bubbling or proper handling.
fn unwrap_with_panic_message<T>(maybe: Result<T, String>) -> T {
    match maybe {
        Ok(definitely) => definitely,
        Err(error) => panic!("{}", error)
    }
}
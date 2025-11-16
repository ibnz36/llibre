use std::{ffi::OsString, fs, io};

pub fn expect_dir(dir_entry: &fs::DirEntry) -> io::Result<()> {
    if dir_entry.file_type()?.is_dir() {
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::NotADirectory,
        "Expected category directory, found file",
    ))
}

pub fn expected_data_in_collections() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "Invalid collection file. Please delete it and re-load all books again.",
    )
}

pub fn osstring_conversion_error(_: OsString) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidFilename,
        "Failed to convert file name to String",
    )
}

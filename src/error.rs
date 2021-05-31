// least - error.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

pub type Result<T> = std::result::Result<T, ErrorKind>;

pub enum ErrorKind {
    Io(std::io::Error),
    Fmt,
    Unknown,
}

impl From<std::io::Error> for ErrorKind {
    fn from(e: std::io::Error) -> Self {
        ErrorKind::Io(e)
    }
}

impl From<crossterm::ErrorKind> for ErrorKind {
    fn from(e: crossterm::ErrorKind) -> Self {
        match e {
            crossterm::ErrorKind::IoError(e) => ErrorKind::Io(e),
            crossterm::ErrorKind::FmtError(_) => ErrorKind::Fmt,
            _ => ErrorKind::Unknown,
        }
    }
}

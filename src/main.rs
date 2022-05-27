// least - main.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::{
    io::stdin,
    ffi::OsStr,
};

use clap::Parser;
use crossterm::{
    style::{
        Color,
        Stylize,
        style
    },
    tty::IsTty
};

mod passthrough;
mod pager;
mod buffer;
mod events;

#[derive(PartialEq)]
pub enum InputMode {
    Stdin,
    File(std::path::PathBuf),
}

// to allow for parsing via clap
impl From<&OsStr> for InputMode {
    fn from(raw: &OsStr) -> Self {
        match raw {
            x if x == OsStr::new("-") => Self::Stdin,
            x => Self::File(std::path::PathBuf::from(x)),
        }
    }
}

fn parse_page_mode(val: bool) -> bool {
    // Determining if we run in passthrough (no paging) mode.
    // This mode streams raw bytes from input to stdout; gotta go fast.
    val || !std::io::stdout().is_tty()
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Don't page output. Useful for passing text through least. Default if stdout is not a TTY
    #[clap(long, parse(from_flag = parse_page_mode))]
    no_page: bool,
    /// File to display. Use - or leave empty to read from stdin
    #[clap(parse(from_os_str), default_value = "-")]
    file: InputMode
}

fn main() {
    let cli = Cli::parse();

    // Handling for least being called in stdin mode without a pipe.
    // Otherwise, `least -` on the terminal will cause complete lockup.
    if cli.file == InputMode::Stdin && stdin().is_tty() {
        eprintln!("Missing input or filename. See 'least --help' for more info.");
        std::process::exit(1);
    }

    // Calling the pager or the passtrough.
    let res = match cli.no_page {
        true => passthrough::run(cli.file),
        false => {
            let r = pager::run(cli.file);
            let mut stdout = std::io::stdout();
            pager::deinit_terminal(&mut stdout);
            r
        },
    };

    // Main error handling logic
    match res {
        Ok(_) => std::process::exit(0),
        Err(kind) => {
            let (text, code) = match kind {
                // error::ErrorKind::Io(e) => (
                //     format!("I/O error occurred: {}", e),
                //     2
                // ),
                // error::ErrorKind::Fmt => (
                //     String::from("Formatting error occurred"),
                //     3
                // ),
                _ => (
                    String::from("Unexpected error occurred"),
                    42
                ),
            };
            if std::io::stderr().is_tty() {
                eprintln!("{}", style(text).with(Color::Red));
            } else {
                eprintln!("{}", text);
            }
            std::process::exit(code)
        }
    };
}

// least - main.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::io::stdin;

use crossterm::{
    tty::IsTty,
    style::{
        style, Color,
    },
};

mod passthrough;
mod pager;
mod buffer;
mod events;
mod error;

fn main() {
    let app = clap::App::new(clap::crate_name!())
                        .version(clap::crate_version!())
                        .author(clap::crate_authors!(", "))
                        .about(clap::crate_description!())
                        .arg(clap::Arg::with_name("NOPAGE")
                                   .long("no-page")
                                   .takes_value(false)
                                   .required(false)
                                   .help("Don't page output. Useful for passing text through least.\nDefault if stdout is not a TTY."))
                        .arg(clap::Arg::with_name("FILE")
                                   .takes_value(true)
                                   .required(false)
                                   .help("File to display"));
    let arg_matches = app.get_matches();

    // Determining if we run in filename or stdin mode.
    let path_or_none = match arg_matches.value_of("FILE") {
        Some(p) if p != "-" => Some(std::path::Path::new(p)),
        Some(_) | None => None,
    };

    // Handling for least being called in stdin mode without a pipe.
    // Otherwise, `least -` on the terminal will cause complete lockup.
    if path_or_none == None && stdin().is_tty() {
        println!("Missing input or filename. See 'least --help' for more info.");
        std::process::exit(1);
    }

    // Determining if we run in passthrough (no paging) mode.
    // This mode streams raw bytes from input to stdout; gotta go fast.
    let passthrough_mode = arg_matches.is_present("NOPAGE") || !std::io::stdout().is_tty();

    // Calling the pager or the passtrough.
    let res = match passthrough_mode {
        true => passthrough::run(path_or_none),
        false => {
            let r = pager::run(path_or_none);
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
                error::ErrorKind::Io(e) => (
                    format!("I/O error occurred: {}", e),
                    2
                ),
                error::ErrorKind::Fmt => (
                    String::from("Formatting error occurred"),
                    3
                ),
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

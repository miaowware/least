// least - main.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::fs::File;
use std::io::{
    BufReader, stdin, stdout
};
use std::path::Path;

use clap::{Arg, App};
use crossterm::{
    Result, tty::IsTty,
};

mod passthrough;
mod pager;
mod buffer;
mod errfmt;
mod events;

fn main() -> Result<()> {
    let m = App::new(clap::crate_name!())
                 .version(clap::crate_version!())
                 .author(clap::crate_authors!(", "))
                 .about(clap::crate_description!())
                 .arg(Arg::with_name("NOPAGE")
                      .long("no-page")
                      .takes_value(false)
                      .required(false)
                      .help("Don't page output. Useful for passing text through least.\nDefault if stdout is not a TTY."))
                 .arg(Arg::with_name("FILE")
                      .takes_value(true)
                      .required(false)
                      .help("File to display"))
                 .get_matches();

    match get_path(m.value_of("FILE")) {
        Some(p) => {
            let input = BufReader::new(match File::open(p) {
                Ok(f) => f,
                Err(e) => errfmt::print_err_exit(e, 1),
            });
            if !m.is_present("NOPAGE") && stdout().is_tty() {
                pager::run_pager(input)?;
            } else {
                passthrough::run_passthrough(input)?;
            }
        },
        None => {
            let input = stdin();
            if !m.is_present("NOPAGE") && stdout().is_tty() {
                pager::run_pager(input.lock())?;
            } else {
                passthrough::run_passthrough(input.lock())?;
            }
        },
    };

    Ok(())
}

fn get_path(s: Option<&str>) -> Option<&Path> {
    match s {
        Some(p) if p != "-" => Some(&Path::new(p)),
        Some(_) | None => None,
    }
}

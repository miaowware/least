// least
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::{fs::File, io::BufRead};
use std::io::{BufReader, stdin};
use std::path::Path;

use clap::{Arg, App};

fn main() {
    let m = App::new(clap::crate_name!())
                 .version(clap::crate_version!())
                 .author(clap::crate_authors!(", "))
                 .about(clap::crate_description!())
                 .arg(Arg::with_name("FILE")
                      .takes_value(true)
                      .required(false)
                      .help("file to display"))
                 .get_matches();

    match get_path(m.value_of("FILE")) {
        Some(p) => process_file(&p),
        None => process_stdin()
    };
}

fn get_path(s: Option<&str>) -> Option<&Path> {
    match s {
        Some(p) if p != "-" => Some(&Path::new(p)),
        Some(_) => None,
        None => None
    }
}

fn process_file(path: &Path) {
    let buffer = BufReader::new(match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    });
    for line in buffer.lines() {
        match line {
            Ok(s) => println!("{}", s),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
    }
}

fn process_stdin() {
    let input = stdin();
    for line in input.lock().lines() {
        match line {
            Ok(s) => println!("{}", s),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
    }
}

// least
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

extern crate clap;
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

    match m.value_of("FILE") {
        Some(f) => println!("Using input file: {}", f),
        None => println!("Using stdin")
    };
}

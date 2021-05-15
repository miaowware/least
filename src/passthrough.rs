// least
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::io::BufRead;
use crossterm::Result;

pub fn run_passthrough<T: BufRead + Sized>(input: T) -> Result<()> {
    for line in input.lines() {
        match line {
            Ok(s) => println!("{}", s),
            Err(e) => crate::errfmt::print_err_exit(e, 1),
        };
    }
    Ok(())
}

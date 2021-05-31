// least - passthrough.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::io::{Read, copy, stdin, stdout};
use std::path::Path;

use crate::error::Result;

fn stream_it_til_theres_none<T: Read>(mut input: T) -> Result<()> {
    let mut stdout = stdout();
    copy(&mut input, &mut stdout)?;
    Ok(())
}

/// **Passthrough entrypoint**
pub fn run(source: Option<&Path>) -> Result<()> {
    match source {
        Some(p) => {
            stream_it_til_theres_none(std::fs::File::open(p)?)
        },
        None => {
            stream_it_til_theres_none(stdin())
        }
    }
}

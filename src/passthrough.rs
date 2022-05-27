// least - passthrough.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::io::{Read, copy, stdin, stdout};

use anyhow;

use crate::InputMode;

fn stream_it_til_theres_none<T: Read>(mut input: T) -> anyhow::Result<()> {
    let mut stdout = stdout();
    copy(&mut input, &mut stdout)?;
    Ok(())
}

/// **Passthrough entrypoint**
pub fn run(source: InputMode) -> anyhow::Result<()> {
    match source {
        InputMode::File(p) => stream_it_til_theres_none(std::fs::File::open(p)?),
        InputMode::Stdin => stream_it_til_theres_none(stdin()),
    }
}

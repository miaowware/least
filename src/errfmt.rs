// least
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use crossterm::{
    ExecutableCommand, style::{self, Colorize}, tty::IsTty
};

pub fn print_err_exit<T: std::error::Error>(error: T, code: i32) -> ! {
    let mut stderr = std::io::stderr();
    if stderr.is_tty() {
        match stderr.execute(style::PrintStyledContent(format!("{}\r\n", error).red())) {
            Ok(_) => std::process::exit(code),
            Err(_) => std::process::exit(69)
        };
    }
    eprintln!("{}", error);
    std::process::exit(code);
}

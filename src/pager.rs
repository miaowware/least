// least - pager.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::{
    io::{BufRead, Write, stdin, stdout, Stdout, BufReader},
    path::Path,
};

use anyhow;
use crossterm::{ExecutableCommand, queue,
    cursor::{
        MoveToNextLine,
        MoveTo,
    },
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode,
        Clear,
        ClearType,
    }};

use crate::{
    buffer,
    events::{
        process_event,
        LeastEvent,
        get_raw_event,
    }
};

/// **Pager init and main loop**
fn do_pager<T: BufRead + Sized>(input: T) -> anyhow::Result<()> {
    let _dummy_buffer: Vec<String> = (0..200).map(|x| format!("{:<3}.#", x)+"....#".repeat(30).as_str()).collect();
    let mut buf = buffer::PagerBuffer{
        // TODO: Only get a certain number of lines
        lines: input.lines().map(|x| x.unwrap_or("".to_string())).collect(),
        // lines: _dummy_buffer,
        row: 0, col: 0
    };

    let mut stdout = stdout();

    // First paint
    draw_screen(&mut stdout, buf.compute_screen(terminal::size()?))?;

    // The main application loop
    // We only do something when we get an event, like keypresses and terminal resizing
    // TODO: `read()` is currently blocking.
    loop {
        let event = get_raw_event()?;

        if let Some(ev) = event {
            match process_event(ev) {
                LeastEvent::Exit => break,
                LeastEvent::ScrollUp => {
                    draw_screen(
                        &mut stdout,
                        buf.scroll(1, buffer::Direction::Up)
                           .compute_screen(terminal::size()?)
                    )?;
                },
                LeastEvent::ScrollDown => {
                    draw_screen(
                        &mut stdout,
                        buf.scroll(1, buffer::Direction::Down)
                           .compute_screen(terminal::size()?)
                    )?;
                },
                LeastEvent::ScrollLeft => {
                    draw_screen(
                        &mut stdout,
                        buf.scroll(1, buffer::Direction::Left)
                           .compute_screen(terminal::size()?)
                    )?;
                },
                LeastEvent::ScrollRight => {
                    draw_screen(
                        &mut stdout,
                        buf.scroll(1, buffer::Direction::Right)
                           .compute_screen(terminal::size()?)
                    )?;
                },
                LeastEvent::Resize(w, h) => {
                    draw_screen(
                        &mut stdout,
                        buf.compute_screen((w, h))
                    )?;
                }
                _ => {},
            };
        } else {
            std::thread::sleep(std::time::Duration::from_millis(25));
        }

        stdout.flush()?;
    }

    Ok(())
}

fn draw_screen(stdout: &mut Stdout, lines: Vec<String>) -> anyhow::Result<()> {
    queue!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
    let mut lns = lines.into_iter();
    write!(stdout, "{}", lns.next().unwrap_or(String::from("")))?;
    for line in lns {
        queue!(stdout, MoveToNextLine(1))?;
        write!(stdout, "{}", line)?;
    }
    stdout.flush()?;
    Ok(())
}

/// **Terminal initialisation**
///
/// This enables raw mode and switches to the alternate buffer.
/// This also adds a panic handler to ensure deinitialisation.
fn init_terminal(stdout: &mut Stdout) -> anyhow::Result<()> {
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // add a hook to panic! to deinitialise the terminal
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let mut out = std::io::stdout();
        deinit_terminal(&mut out);
        default_panic(info);
    }));

    Ok(())
}

/// **Returns the terminal to normal modes**
///
/// No return value/result since this essentially can't be
/// reached in a situation where it would fail.
pub fn deinit_terminal(stdout: &mut Stdout) {
    let _ = disable_raw_mode();
    let _ = stdout.execute(LeaveAlternateScreen);
}

/// **Pager entrypoint**
pub fn run(source: Option<&Path>) -> anyhow::Result<()> {
    let mut stdout = stdout();
    match source {
        Some(p) => {
            init_terminal(&mut stdout)?;
            do_pager(BufReader::new(std::fs::File::open(p)?))
        },
        None => {
            let input = stdin();
            init_terminal(&mut stdout)?;
            do_pager(input.lock())
        }
    }
}

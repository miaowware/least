// least - pager.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::{
    io::{Write, stdin, stdout, Stdout, BufRead},
    fs::File, thread, sync::mpsc::{self, TryRecvError}, time::Duration, panic,
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
    }, BufferSource
};

/// **Pager init and main loop**
fn do_pager(mut buf: buffer::PagerBuffer, source: BufferSource) -> anyhow::Result<()> {
    let mut stdout = stdout();
    let thread_stuff = match source {
        BufferSource::Stdin => {
            let chan = mpsc::channel::<String>();
            let t = buf.target.clone();
            let handle = thread::spawn(|| {
                let target = t;
                let tx = chan.0;
                let stdin = stdin().lock();
                let count: usize = 0;
                let mut lines = stdin.lines();
                loop {
                    if count < target.load(std::sync::atomic::Ordering::Relaxed) {
                        let line = lines.next();
                        match line {
                            Some(Ok(s)) => tx.send(s).unwrap(),
                            Some(Err(e)) => panic!("{}", e),
                            None => break,
                        }
                    } else {
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            });
            Some((chan.1, handle))
        },
        BufferSource::File(_) => None
    };

    // Init buffer target
    if !buf.reached_eof {
        buf.update_target(terminal::size()?.1);
    }

    // First paint
    draw_screen(&mut stdout, buf.compute_screen(terminal::size()?))?;

    // The main application loop
    // We only do something when we get an event, like keypresses and terminal resizing
    // TODO: `read()` is currently blocking.
    'mainloop: loop {
        if !buf.reached_eof {
            if let Some((ref rx, ref _handle)) = thread_stuff {
                'receive: loop {
                    match rx.try_recv() {
                        Ok(s) => {
                            buf.lines.push(s);
                            draw_screen(&mut stdout, buf.compute_screen(terminal::size()?))?;
                        },
                        Err(e) => match e {
                            TryRecvError::Empty => break 'receive,
                            TryRecvError::Disconnected => {
                                // TODO: add checks for receiver thread panic and such
                                buf.reached_eof = true;
                                break 'receive;
                            }
                        },
                    }
                }
            }
        }

        let event = get_raw_event()?;

        if let Some(ev) = event {
            match process_event(ev) {
                LeastEvent::Exit => break 'mainloop,
                LeastEvent::ScrollUp => {
                    draw_screen(
                        &mut stdout,
                        buf.scroll(1, buffer::Direction::Up)
                           .compute_screen(terminal::size()?)
                    )?;
                },
                LeastEvent::ScrollDown => {
                    let term_size = terminal::size()?;
                    draw_screen(
                        &mut stdout,
                        buf.scroll(1, buffer::Direction::Down)
                           .update_target(term_size.1)
                           .compute_screen(term_size)
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
fn deinit_terminal(stdout: &mut Stdout) {
    let _ = disable_raw_mode();
    let _ = stdout.execute(LeaveAlternateScreen);
}

/// **Pager entrypoint**
pub fn run(source: BufferSource) -> anyhow::Result<()> {
    let mut stdout = stdout();
    init_terminal(&mut stdout)?;
    let buffer = match &source {
        BufferSource::File(p) => buffer::PagerBuffer::try_from(File::open(p)?)?,
        BufferSource::Stdin => buffer::PagerBuffer::default(),
    };
    let r = do_pager(buffer, source);
    deinit_terminal(&mut stdout);
    r
}

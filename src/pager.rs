// least - pager.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::io::{
    BufRead, Write, stdout, Stdout
};
use crossterm::{ExecutableCommand, Result, queue,
    cursor::{
        MoveToNextLine,
        MoveTo,
    },
    event::{
        self,
        Event,
        KeyEvent,
        KeyCode,
        KeyModifiers,
    }, terminal, terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode,
        Clear,
        ClearType,
    }};

use crate::buffer;

pub fn run_pager<T: BufRead + Sized>(input: T) -> Result<()> {
    let _dummy_buffer: Vec<String> = (0..200).map(|x| format!("{:<3}.#", x)+"....#".repeat(30).as_str()).collect();
    let mut buf = buffer::PagerBuffer{
        // TODO: Only get a certain number of lines
        lines: input.lines().map(|x| x.unwrap_or("".to_string())).collect(),
        // lines: _dummy_buffer,
        row: 0, col: 0
    };

    let mut stdout = stdout();
    let quit_key = KeyEvent{code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE};
    let up_key = KeyEvent{code: KeyCode::Up, modifiers: KeyModifiers::NONE};
    let down_key = KeyEvent{code: KeyCode::Down, modifiers: KeyModifiers::NONE};
    let left_key = KeyEvent{code: KeyCode::Left, modifiers: KeyModifiers::NONE};
    let right_key = KeyEvent{code: KeyCode::Right, modifiers: KeyModifiers::NONE};

    init_terminal(&mut stdout)?;

    draw_screen(&mut stdout, buf.compute_screen(terminal::size()?))?;

    // The main application loop
    // We only do something when we get an event, like keypresses and terminal resizing
    // In the meantime, `read()` is blocking.
    loop {
        match event::read()? {
            Event::Key(ev) if ev == quit_key => break,
            Event::Key(ev) if ev == up_key => {
                draw_screen(&mut stdout, buf.scroll(1, buffer::Direction::Up).compute_screen(terminal::size()?))?;
            },
            Event::Key(ev) if ev == down_key => {
                draw_screen(&mut stdout, buf.scroll(1, buffer::Direction::Down).compute_screen(terminal::size()?))?;
            },
            Event::Key(ev) if ev == left_key => {
                draw_screen(&mut stdout, buf.scroll(1, buffer::Direction::Left).compute_screen(terminal::size()?))?;
            },
            Event::Key(ev) if ev == right_key => {
                draw_screen(&mut stdout, buf.scroll(1, buffer::Direction::Right).compute_screen(terminal::size()?))?;
            },
            Event::Key(ev) => write!(&mut stdout, "Key: {:?}", ev)?,
            Event::Mouse(ev) => write!(&mut stdout, "Mouse: {:?}", ev)?,
            Event::Resize(w, h) => write!(&mut stdout, "Resize: {}x{}", w, h)?,
            // Event::Resize(w, h) => draw_screen(buf.compute_screen((w, h)))?,
        };
        stdout.flush()?;
    }

    deinit_terminal(&mut stdout)?;

    Ok(())
}

fn draw_screen(stdout: &mut Stdout, lines: Vec<String>) -> Result<()> {
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

fn init_terminal(stdout: &mut Stdout) -> Result<()> {
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    //stdout.execute(Hide)?;

    // add a hook to panic! to deinitialise the terminal
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let mut out = std::io::stdout();
        deinit_terminal(&mut out).unwrap_or(());
        default_panic(info);
    }));

    Ok(())
}

fn deinit_terminal(stdout: &mut Stdout) -> Result<()> {
    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}

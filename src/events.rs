// least - events.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use crossterm::event::{
    Event,
    KeyEvent,
    KeyCode,
    KeyModifiers,
    MouseEventKind,
};

pub enum LeastEvent {
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    Exit,
    Nop,
    Resize(u16, u16),
}

pub fn process_event(ev: Event) -> LeastEvent {
    // TODO: this shouldn't be hard-coded
    let quit_key = KeyEvent{code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE};
    let up_key = KeyEvent{code: KeyCode::Up, modifiers: KeyModifiers::NONE};
    let down_key = KeyEvent{code: KeyCode::Down, modifiers: KeyModifiers::NONE};
    let left_key = KeyEvent{code: KeyCode::Left, modifiers: KeyModifiers::NONE};
    let right_key = KeyEvent{code: KeyCode::Right, modifiers: KeyModifiers::NONE};

    match ev {
        Event::Key(ke) => match ke {
            _ if ke == quit_key => LeastEvent::Exit,
            _ if ke == up_key => LeastEvent::ScrollUp,
            _ if ke == down_key => LeastEvent::ScrollDown,
            _ if ke == left_key => LeastEvent::ScrollLeft,
            _ if ke == right_key => LeastEvent::ScrollRight,
            _ => LeastEvent::Nop,
        },
        Event::Mouse(me) => match me.kind {
            MouseEventKind::ScrollUp => LeastEvent::ScrollUp,
            MouseEventKind::ScrollDown => LeastEvent::ScrollDown,
            _ => LeastEvent::Nop,
        },
        Event::Resize(w, h) => LeastEvent::Resize(w, h),
    }
}

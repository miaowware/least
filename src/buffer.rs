// least - buffer.rs
// ---
// Copyright 2021 classabbyamp, 0x5c
// Released under the terms of the BSD 3-Clause license.

use std::{
    cmp, io::{BufReader, BufRead}, fs::File,
    sync::{atomic::AtomicUsize, Arc},
};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct PagerBuffer {
    // internal buffer
    pub lines: Vec<String>,
    // index of the topmost row of the current screen
    pub row: usize,
    // index of the leftmost column of the current screen
    pub col: usize,
    // target line number for the reader thread
    // TODO: wrap this for platforms without atomics
    pub target: Arc<AtomicUsize>,
    // if the input end has been reached
    pub reached_eof: bool,
}

impl PagerBuffer {
    /// shortcut to the `length` of the internal buffer `lines`
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// scrolls the screen around with an increment `skip` in direction `direction`
    pub fn scroll(&mut self, skip: usize, direction: Direction) -> &mut Self {
        match direction {
            Direction::Up => {
                self.row = match self.row.checked_sub(skip) {
                    Some(0) | None => 0,
                    Some(n) => n,
                }
            },
            Direction::Down => {
                // TODO: make it so can't go down further than last line at bottom
                self.row = match self.row.checked_add(skip) {
                    Some(n) if n < self.len() => n,
                    Some(_) | None => self.len() - 1,
                }
            },
            Direction::Left => {
                self.col = match self.col.checked_sub(skip) {
                    Some(0) | None => 0,
                    Some(n) => n,
                }
            },
            Direction::Right => {
                self.col = match self.col.checked_add(skip) {
                    Some(n) => n,
                    None => self.col,
                }
            },
        };
        self
    }

    /// Generates the lines of text for the current screen based on the terminal size and buffer contents
    pub fn compute_screen(&self, term_size: (u16, u16)) -> Vec<String> {
        let cols = term_size.0 as usize;
        let rows = term_size.1 as usize;

        let len_to_eob = self.len() - self.row;

        match &len_to_eob.cmp(&rows) {
            cmp::Ordering::Greater => {
                // clip at term size
                self.lines[self.row..(self.row + rows)].into_iter()
                .map(|x| clip_string(x.clone(), &self.col, &cols))
                .collect()
            },
            cmp::Ordering::Equal => {
                // pass on
                // wtaf is that range - abby
                // hell, we might never know - 5c
                self.lines[self.row..=(self.row + rows - 1)].into_iter()
                .map(|x| clip_string(x.clone(), &self.col, &cols))
                .collect()
            },
            cmp::Ordering::Less => {
                // add tilde rows
                // only allowed if full buffer size is less than terminal size
                // or if terminal embiggened
                let mut text: Vec<String> = self.lines[self.row..(self.row + len_to_eob)].into_iter()
                                                          .map(|x| clip_string(x.clone(), &self.col, &cols))
                                                          .collect();
                text.resize(rows + 1, String::from("~"));
                text
            },
        }
    }

    /// Sets the target to the appropriate value based on terminal height
    /// For performance reasons, only call this when scrolling downwards
    pub fn update_target(&mut self, term_height: u16) -> &mut Self {
        if self.reached_eof {
            return self;
        }
        let readahead = self.len() + ((term_height as usize) * 4);
        self.target.fetch_max(readahead, std::sync::atomic::Ordering::Relaxed);
        self
    }
}

impl Default for PagerBuffer {
    fn default() -> Self {
        Self{
            lines: vec![], row: 0, col: 0,
            reached_eof: false, target: Arc::new(AtomicUsize::new(0))
        }
    }
}

impl TryFrom<File> for PagerBuffer {
    type Error = anyhow::Error;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let buffered = BufReader::new(file);
        // TODO: Byebye unwrap (also, non-utf8 input)
        let lines = buffered.lines().map(|x| x.unwrap_or(String::new())).collect();
        Ok(Self{lines, reached_eof: true, ..Default::default()})
    }
}

fn clip_string(s: String, col: &usize, cols: &usize) -> String {
    // see if the line is longer than the leftmost column
    match &s.len().cmp(col) {
        cmp::Ordering::Greater | cmp::Ordering::Equal => {
            let mut s_ = String::from(s.split_at(col.clone()).1);
            s_.truncate(cols.clone());
            s_
        },
        cmp::Ordering::Less => String::new(),
    }
}

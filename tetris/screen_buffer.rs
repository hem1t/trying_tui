use std::io::{stdout, Result};
use std::ops::Deref;

use crossterm::cursor::MoveTo;
use crossterm::execute;

impl Deref for ScreenBuffer {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

pub struct ScreenBuffer {
    cells: usize,
    lines: usize,
    buffer: Vec<Vec<char>>,
}

impl ScreenBuffer {
    pub fn lines(&self) -> usize {
        self.lines
    }
    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn new((c, l): (usize, usize)) -> Self {
        Self {
            cells: c,
            lines: l,
            buffer: vec![vec![' '; c]; l],
        }
    }

    pub fn auto_new() -> Result<Self> {
        let (c, l) = crossterm::terminal::size()?;
        Ok(Self {
            cells: c as usize,
            lines: l as usize,
            buffer: vec![vec![' '; c as usize]; l as usize],
        })
    }

    pub fn get_mut(&mut self, (c, l): (usize, usize)) -> Option<&mut char> {
        Some(self.buffer.get_mut(l)?.get_mut(c)?)
    }
    pub fn get(&self, (c, l): (usize, usize)) -> Option<&char> {
        Some(self.buffer.get(l)?.get(c)?)
    }

    pub fn flush(&self) -> Result<()> {
        for (l, line) in self.buffer.iter().enumerate() {
            for (c, ch) in line.iter().enumerate() {
                execute!(stdout(), MoveTo(c as u16, l as u16))?;
                print!("{ch}");
            }
        }
        Ok(())
    }
}

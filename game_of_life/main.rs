use std::{
    io::{stdout, Write},
    ops::Deref,
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

static BLOCK_1: &str = "\u{2588}\u{2588}";
static BLOCK_0: &str = "  ";

fn main() -> std::io::Result<()> {
    let mut screen = Screen::create();
    let mut judge_screen = Screen::create();
    // Debug line
    for l in 0..5 {
        for r in 0..5 {
            screen.set(l, r);
        }
    }
    // dbg!(crossterm::terminal::size()?);
    // dbg!(&screen.len());
    // dbg!(&screen[0].len());

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    // let mut counter = 0;
    loop {
        draw_screen(&screen)?;
        // execute!(stdout(), cursor::MoveTo(0, screen.len() as u16))?;
        // print!("{counter}");
        // stdout().flush()?;
        // counter += 1;
        if poll(Duration::from_millis(750)).unwrap() {
            match read()? {
                Event::Key(k) if k == KeyCode::Char('q').into() => break,
                _ => (),
            }
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn draw_screen(screen: &Screen) -> std::io::Result<()> {
    for (ln, row) in screen.iter().enumerate() {
        execute!(stdout(), cursor::MoveTo(0, ln as u16))?;
        for &alive in row {
            if alive {
                print!("{BLOCK_1}");
            } else {
                print!("{BLOCK_0}");
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Screen {
    s: Vec<Vec<bool>>,
}

impl Screen {
    fn create() -> Self {
        if let Ok((r, l)) = crossterm::terminal::size() {
            return Screen {
                s: vec![vec![false; r as usize / 2]; l as usize],
            };
        }
        Screen {
            s: vec![vec![true; 5]; 5],
        }
    }

    fn get(&self, l: usize, r: usize) -> bool {
        self.s[l][r]
    }

    fn set(&mut self, l: usize, r: usize) {
        self.s[l][r] = true;
    }

    fn unset(&mut self, l: usize, r: usize) {
        self.s[l][r] = false;
    }
}

impl Deref for Screen {
    type Target = Vec<Vec<bool>>;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

use std::{io::stdout, ops::Deref, thread, time::Duration};

use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

static BLOCK: &str = "\u{2588}\u{2588}";

fn main() -> std::io::Result<()> {
    let mut screen = Screen::create();
    for l in 0..5 {
        for r in 0..5 {
            screen.set(l, r);
        }
    }
    dbg!(crossterm::terminal::size()?);
    dbg!(&screen.len());
    dbg!(&screen[0].len());

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    draw_screen(&screen)?;
    thread::sleep(Duration::from_millis(1700));
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn draw_screen(screen: &Screen) -> std::io::Result<()> {
    // TODO: requires controlled printing
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    for row in screen.iter() {
        for &alive in row {
            if alive {
                print!("{BLOCK}");
            } else {
                print!("  ");
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

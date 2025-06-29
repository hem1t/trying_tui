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
    // Debug line
    // for l in (screen.l / 2 - 5)..=((screen.l / 2) + 10) {
    //     for r in (screen.l / 2 - 5)..=((screen.r / 2) + 10) {
    //         screen.set(l, r);
    //     }
    // }

    // dbg!(crossterm::terminal::size()?);
    // dbg!(&screen.len());
    // dbg!(&screen[0].len());

    // an example picked from wikipedia
    let (l, r) = (90, 50);
    screen.set(l, r);
    screen.set(l, r + 1);
    screen.set(l + 1, r + 1);
    screen.set(l + 1, r + 2);
    screen.set(l - 1, r + 1);

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    // let mut counter = 0;
    loop {
        draw_screen(&screen)?;
        // execute!(stdout(), cursor::MoveTo(0, screen.len() as u16))?;
        // print!("{counter}");
        // stdout().flush()?;
        // counter += 1;
        screen = to_next_life(&screen);
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

fn to_next_life(from: &Screen) -> Screen {
    let mut judge_screen = from.clone();

    for (ln, row) in from.iter().enumerate() {
        for (rn, &alive) in row.iter().enumerate() {
            let count = from.neighbors_alive(ln, rn);
            if alive && count < 2 || count > 3 {
                judge_screen.unset(ln, rn);
            } else if !alive && count == 3 {
                judge_screen.set(ln, rn);
            }
        }
    }
    judge_screen
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

#[derive(Debug, Clone)]
struct Screen {
    l: usize,
    r: usize,
    s: Vec<Vec<bool>>,
}

impl Screen {
    fn create() -> Self {
        if let Ok((r, l)) = crossterm::terminal::size() {
            return Screen {
                l: l as usize,
                r: r as usize / 2,
                s: vec![vec![false; r as usize / 2]; l as usize],
            };
        }
        Screen {
            l: 5,
            r: 5,
            s: vec![vec![true; 5]; 5],
        }
    }

    fn neighbors_alive(&self, l: usize, r: usize) -> usize {
        let mut alives = 0;

        let (rnz, lnz) = (r != 0, l != 0);
        let (rib, lib) = (r + 1 < self.r, l + 1 < self.l);

        if lib && self.get(l + 1, r) {
            alives += 1;
        }
        if lnz && self.get(l - 1, r) {
            alives += 1;
        }
        if rib && self.get(l, r + 1) {
            alives += 1;
        }
        if rnz && self.get(l, r - 1) {
            alives += 1;
        }
        if lib && rib && self.get(l + 1, r + 1) {
            alives += 1;
        }
        if lnz && rnz && self.get(l - 1, r - 1) {
            alives += 1;
        }
        if lib && rnz && self.get(l + 1, r - 1) {
            alives += 1;
        }
        if lnz && rib && self.get(l - 1, r + 1) {
            alives += 1;
        }
        alives
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

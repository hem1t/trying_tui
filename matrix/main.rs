// TODO: put screen in between
use std::{io::stdout, ops::DerefMut, time::Duration};

use crossterm::{
    cursor::{self, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use rand::{
    distr::{Distribution, Uniform},
    Rng,
};

static FRAME_TIME: Duration = Duration::from_millis(9);

fn main() -> std::io::Result<()> {
    let (ci, li) = crossterm::terminal::size()?;

    // lines for each column
    let mut lines: Vec<Line> = (0..ci / 2)
        .map(|col| {
            Line::new(
                size_rng(li.into()),
                speed_rng(),
                col as usize * 2,
                li.into(),
            )
        })
        .collect();

    // Inits
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), cursor::Hide)?;

    'main_loop: loop {
        if poll(FRAME_TIME)? {
            match read()? {
                Event::Key(k) if k == KeyCode::Char('q').into() => break 'main_loop,
                _ => (),
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    execute!(stdout(), cursor::Show)?;
    Ok(())
}

fn speed_rng() -> usize {
    rand::rng().random_range(1..=10)
}

fn size_rng(max: usize) -> usize {
    // WARNING: perform mul & div one by one
    let start = max / 3;
    let end = (max / 5) * 4;
    rand::rng().random_range(start..=end)
}

// TODO: unchanging data; should exists in `Line`
fn choose_color(li: usize, ls: usize) -> Color {
    if li == ls - 1 {
        return Color::White;
    }
    if li >= ls {
        return Color::Rgb { r: 0, g: 0, b: 0 };
    }

    let unit = std::cmp::max(255 / ls, 20);
    let g = std::cmp::min(unit * li, 255) as u8;

    Color::Rgb { r: 0, g, b: 0 }
}

// matrix line vertical
// has speed of it's own
//
// Matrix stores update counter
// when 0 return Some or None

struct Line {
    size: usize,
    speed: usize,
    passed: usize,
    col: usize,
    pos: usize,
    line: Vec<char>,
}

impl Line {
    fn new(size: usize, speed: usize, col: usize, l_size: usize) -> Self {
        let kata_rng = Uniform::new_inclusive(0x30A1, 0x30FD).unwrap();
        let rng = rand::rng();
        let mut line = kata_rng
            .sample_iter(rng)
            .take(size as usize)
            .map(|i| unsafe { char::from_u32_unchecked(i) })
            .collect::<Vec<char>>();

        line.extend(vec![' '; l_size - size]);

        Self {
            size,
            pos: 0,
            passed: speed,
            speed,
            col,
            line,
        }
    }
}

impl std::ops::Deref for Line {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.line
    }
}

impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.line
    }
}

// TODO: put screen in between
use std::{
    io::{stdout, Write},
    ops::DerefMut,
    time::Duration,
};

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

///
/// per frame wait after draw, for an event, before continuing to next frame
static FRAME_TIME: Duration = Duration::from_millis(9);
///
/// After how many frames to change the position of a line
static SPEED_RANGE: (usize, usize) = (2, 10);
///
/// First number is multiplier and second is divider
/// for MIN and MAX size of lines
static LINE_MIN_RATIO: (usize, usize) = (1, 3);
static LINE_MAX_RATIO: (usize, usize) = (4, 5);

fn main() -> std::io::Result<()> {
    let (ci, li) = crossterm::terminal::size()?;
    let kata_rng = Uniform::new_inclusive(0x30A1, 0x30FD).unwrap();
    let mut matrix: Vec<Line> = (0..ci / 2)
        .map(|_| Line::new(kata_rng, li as usize))
        .collect();
    let mut frame_count = 0;

    // Inits
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), cursor::Hide)?;

    'main_loop: loop {
        for (col, line) in matrix.iter().enumerate() {
            draw_line(line, col as u16 * 2)?;
        }
        stdout().flush()?;

        for mut line in matrix.iter_mut() {
            update_line(&mut line, frame_count);
        }
        frame_count += 1;

        if poll(FRAME_TIME)? {
            match read()? {
                Event::Key(k) if k == KeyCode::Char('q').into() => break 'main_loop,
                _ => (),
            }
        }
    }

    // TODO: clean up,
    //       need to make sure runs even after panic
    //       or before panic
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    execute!(stdout(), cursor::Show)?;
    Ok(())
}

fn update_line(line: &mut Line, frame_count: usize) {
    if frame_count % line.speed.max(1) == 0 {
        line.pos += 1;
    }
}

fn draw_line(line: &Line, on_col: u16) -> std::io::Result<()> {
    let (_, ls) = crossterm::terminal::size()?;
    let pos = line.pos;
    for (row, &(ch, color)) in line.iter().enumerate() {
        let row = (row + pos) % ls as usize;
        execute!(stdout(), MoveTo(on_col, row as u16), Print(ch.with(color)))?;
    }

    Ok(())
}

fn speed_rng() -> usize {
    rand::rng().random_range((SPEED_RANGE.0)..=(SPEED_RANGE.1))
}

fn size_rng(max: usize) -> usize {
    // WARNING: perform mul & div one by one
    let start = LINE_MIN_RATIO.0 * max / LINE_MIN_RATIO.1;
    let end = LINE_MAX_RATIO.0 * max / LINE_MAX_RATIO.1;
    rand::rng().random_range(start..=end)
}

struct Line {
    line: Vec<(char, Color)>,
    speed: usize,
    pos: usize,
}

impl Line {
    fn new<T>(char_rng: T, l_size: usize) -> Self
    where
        T: Distribution<u32>,
    {
        let size: usize = size_rng(l_size);
        let speed: usize = speed_rng();

        let line = char_rng
            .sample_iter(rand::rng())
            .take(size as usize)
            .enumerate()
            .map(|(i, cui)| {
                (
                    unsafe { char::from_u32_unchecked(cui) },
                    choose_color(i as usize, size),
                )
            })
            .collect::<Vec<(char, Color)>>();

        Self {
            line,
            speed,
            pos: 0,
        }
    }
}

impl std::ops::Deref for Line {
    type Target = Vec<(char, Color)>;

    fn deref(&self) -> &Self::Target {
        &self.line
    }
}

impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.line
    }
}

fn choose_color(li: usize, ls: usize) -> Color {
    if li == ls - 1 {
        return Color::White;
    }

    let unit = std::cmp::max(255 / ls, 2);
    let g = std::cmp::min(unit * li, 255) as u8;

    Color::Rgb { r: 0, g, b: 0 }
}

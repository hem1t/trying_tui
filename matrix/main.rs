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

fn speed_rng() -> usize {
    rand::rng().random_range(1..=10)
}

fn size_rng(max: usize) -> usize {
    // WARNING: divide first then multiply
    let start = max / 3;
    let end = (max / 5) * 4;
    rand::rng().random_range(start..=end)
}

fn main() -> std::io::Result<()> {
    let kata_rng = Uniform::new_inclusive(0x30A1, 0x30FD).unwrap();
    let mut rng = rand::rng();
    let frame_time = Duration::from_millis(9);
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
        draw_lines(&lines)?;
        update_pos(&mut lines);

        for line in lines.iter_mut() {
            let ls = line.size;
            line[rand::rng().random_range(0..ls)] =
                unsafe { char::from_u32_unchecked(kata_rng.sample(&mut rng)) };
            line[rand::rng().random_range(0..ls)] =
                unsafe { char::from_u32_unchecked(kata_rng.sample(&mut rng)) };
            line[rand::rng().random_range(0..ls)] =
                unsafe { char::from_u32_unchecked(kata_rng.sample(&mut rng)) };
            line[rand::rng().random_range(0..ls)] =
                unsafe { char::from_u32_unchecked(kata_rng.sample(&mut rng)) };
            line[rand::rng().random_range(0..ls)] =
                unsafe { char::from_u32_unchecked(kata_rng.sample(&mut rng)) };
        }

        if poll(frame_time)? {
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

fn update_pos(lines: &mut Vec<Line>) {
    let l = crossterm::terminal::size().unwrap().1 as usize;
    for line in lines {
        line.passed -= 1;
        if line.passed == 0 {
            line.passed = line.speed;
            line.pos += 1;
        }
        if line.pos == l {
            line.pos = 0;
        }
    }
}

fn draw_lines(lines: &Vec<Line>) -> std::io::Result<()> {
    let l = crossterm::terminal::size()?.1 as usize;
    for line in lines {
        for li in 0..line.len() {
            let l = (li + line.pos) % l;

            let ch = line[li]
                .with(choose_color(li, line.size))
                .on(Color::Rgb { r: 0, g: 0, b: 0 });

            execute!(stdout(), MoveTo(line.col as u16, l as u16), Print(ch))?;
        }
    }
    Ok(())
}

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

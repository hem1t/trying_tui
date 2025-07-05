use screen_buffer::ScreenBuffer;
use tetris_box::make_rect;
mod screen_buffer;
mod tetris_box;

fn main() -> std::io::Result<()> {
    let mut screen = ScreenBuffer::auto_new()?;
    do_something(&mut screen);
    make_rect(&mut screen, (0, 1), (20, 10));
    screen.flush()?;
    Ok(())
}

pub fn do_something(screen: &mut ScreenBuffer) {
    let msg = "hello world!\n\r\tlajsdfkjasdfl;qwueropuiqweropqwuerpkalnzdgaiopsydriqwowyuerqiopweruyqweiopryuqweryioqweuiryqwriou ehllw";
    for (i, c) in msg.chars().enumerate() {
        // cuts out overflow
        // ScreenBuffer doesn't care about spacechars char
        // as it prints every char to it's specific position
        // therefore "\n\r\t" becomes "   " (3 spaces)
        if let Some(ch) = screen.get_mut((i + 4, 5)) {
            *ch = c;
        }
    }
}

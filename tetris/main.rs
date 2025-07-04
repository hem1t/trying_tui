use screen_buffer::ScreenBuffer;

mod screen_buffer;

static BLOCK_CHAR: [&str; 2] = ["  ", "\u{2588}\u{2588}"];

fn main() -> std::io::Result<()> {
    let mut screen = ScreenBuffer::auto_new()?;
    do_something(&mut screen);
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
        if let Some(ch) = screen.get_mut((i, 5)) {
            *ch = c;
        }
    }
}

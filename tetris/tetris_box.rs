use crate::screen_buffer::ScreenBuffer;

#[non_exhaustive]
struct BoxChar;

impl BoxChar {
    pub const TOP_LEFT: char = unsafe { char::from_u32_unchecked(0x250c) };
    pub const TOP_RIGHT: char = unsafe { char::from_u32_unchecked(0x2510) };
    pub const BOTTOM_LEFT: char = unsafe { char::from_u32_unchecked(0x2514) };
    pub const BOTTOM_RIGHT: char = unsafe { char::from_u32_unchecked(0x2518) };
    pub const HORIZONTAL: char = unsafe { char::from_u32_unchecked(0x2500) };
    pub const VERTICAL: char = unsafe { char::from_u32_unchecked(0x2502) };
}

pub fn make_rect(screen: &mut ScreenBuffer, start: (usize, usize), end: (usize, usize)) {
    let (ec, el) = end;
    let (sc, sl) = start;

    for c in 1..ec {
        *screen.get_mut((c, sl)).unwrap() = BoxChar::HORIZONTAL;
        *screen.get_mut((c, el)).unwrap() = BoxChar::HORIZONTAL;
    }
    for l in 1..el {
        *screen.get_mut((sc, l)).unwrap() = BoxChar::VERTICAL;
        *screen.get_mut((ec, l)).unwrap() = BoxChar::VERTICAL;
    }

    *screen.get_mut((sc, sl)).unwrap() = BoxChar::TOP_LEFT;
    *screen.get_mut((ec, sl)).unwrap() = BoxChar::TOP_RIGHT;
    *screen.get_mut((sc, el)).unwrap() = BoxChar::BOTTOM_LEFT;
    *screen.get_mut((ec, el)).unwrap() = BoxChar::BOTTOM_RIGHT;
}

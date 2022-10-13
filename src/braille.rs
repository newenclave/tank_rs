const MIN: u8 = 0x00;
const BASE: u32 = 0x2800;
const DOTS: &'static [u8] = &[ 0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80];

pub fn empty() -> u8 {
    MIN
}

pub fn set_dot(src: u8, x: usize, y:usize) -> u8 {
    let id = x * 4 + y;
    if id < DOTS.len() {
        src | DOTS[id]
    } else {
        src
    }
}

pub fn clean_dot(src: u8, x: usize, y:usize) -> u8 {
    let id = x * 4 + y;
    if id < DOTS.len() {
        src & !DOTS[id]
    } else {
        src
    }
}

pub fn to_char(val: u8) -> char {
    unsafe { char::from_u32_unchecked(val as u32 + BASE) }
}

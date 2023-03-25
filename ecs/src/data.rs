#[derive(Debug)]
pub struct Strength {
    pub s: i16,
    pub h: i16,
}

#[derive(Debug, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Dir {
    pub vx: i32,
    pub vy: i32,
}

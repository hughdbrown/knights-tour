use std::fmt;

pub const NUM_ROWS: i8 = 8;
pub const NUM_COLS: i8 = NUM_ROWS;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct Pos(pub i8, pub i8);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub const OFFSETS: [Pos; 8] = [
    Pos(-2i8, -1i8),
    Pos(-1i8, -2i8),
    Pos( 2i8, -1i8),
    Pos( 1i8, -2i8),
    Pos(-2i8,  1i8),
    Pos(-1i8,  2i8),
    Pos( 2i8,  1i8),
    Pos( 1i8,  2i8),
];




use std::fmt;
use std::collections::{
    HashMap,
    HashSet,
};

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

pub fn calc_visitable(pos: &Pos)-> Vec<Pos>
{
    let mut visitable: Vec<Pos> = Vec::new();
    for offset in OFFSETS.iter() {
        let x: i8 = pos.0 + offset.0;
        if (0 <= x) && (x < NUM_ROWS) {
            let y: i8 = pos.1 + offset.1;
            if (0 <= y) && (y < NUM_COLS) {
                visitable.push(Pos(x, y));
            }
        }
    }
    visitable
}

pub fn calc_squares() -> HashSet<Pos> {
    let mut squares: HashSet<Pos> = HashSet::new();
    for row in 0i8..(NUM_ROWS as i8) {
        for col in 0i8..(NUM_COLS as i8) {
            let pos: Pos = Pos(row, col);
            squares.insert(pos);
        }
    }
    squares
}

pub fn calc_possible() -> HashMap<Pos, Vec<Pos>> {
    let mut possible: HashMap<Pos, Vec<Pos>> = HashMap::new();
    for row in 0i8..(NUM_ROWS as i8) {
        for col in 0i8..(NUM_COLS as i8) {
            let pos: Pos = Pos(row, col);
            let visitable = calc_visitable(&pos);
            possible.insert(pos, visitable);
        }
    }
    possible
}

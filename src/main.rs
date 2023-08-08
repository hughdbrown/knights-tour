use std::collections::{
    HashMap,
    HashSet,
};

mod tour;
mod utils;

use tour::{
    find_tour,
    print_tour,
    SearchResult::{
        Found,
        NotFound,
    },
};
use utils::{
    Pos,
    OFFSETS,
    NUM_ROWS,
    NUM_COLS,
};


fn calc_visitable(pos: &Pos)-> Vec<Pos> 
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


fn main() {
    let mut squares: HashSet<Pos> = HashSet::new();
    for row in 0i8..(NUM_ROWS as i8) {
        for col in 0i8..(NUM_COLS as i8) {
            let pos: Pos = Pos(row, col);
            squares.insert(pos);
        }
    }

    let mut possible: HashMap<Pos, Vec<Pos>> = HashMap::new();
    for row in 0i8..(NUM_ROWS as i8) {
        for col in 0i8..(NUM_COLS as i8) {
            let pos: Pos = Pos(row, col);
            let visitable = calc_visitable(&pos);
            possible.insert(pos, visitable);
        }
    }
    //println!("{:#?}", possible);
    //println!("{:?}", possible);

    let mut tour: Vec<Pos> = Vec::new();
    let start_pos: Pos = Pos(0i8, 0i8);
    tour.push(start_pos);

    match find_tour(
        &start_pos,
        &mut squares, // Candidate squares left to visit 
        &possible, // Table of squares a knight may travel to from that square
        &mut tour, // Knight's tour if successful
    ) {
        Found => {
            println!("Solution found");
            //println!("{:#?}", tour); // Pretty-print the tour
            println!("Length of tour: {}", tour.len());
            print_tour(&tour);
        },
        NotFound => {
            println!("Solution not found");
        },
    }
}

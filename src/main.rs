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
    calc_squares,
    calc_possible,
};


fn main() {
    let mut squares: HashSet<Pos> = calc_squares();
    let possible: HashMap<Pos, Vec<Pos>> = calc_possible();
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

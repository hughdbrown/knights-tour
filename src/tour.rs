use std::collections::{
    HashMap,
    HashSet,
};

use crate::utils::{
    Pos,
    NUM_ROWS,
    NUM_COLS,
};

pub enum SearchResult {
    Found,
    NotFound,
}

pub fn print_tour(tour: &Vec<Pos>) {
    let mut map = [[0; NUM_COLS as usize]; NUM_ROWS as usize];
    for (i, p) in tour.iter().enumerate() {
        let (x, y) = (p.0, p.1);
        map[x as usize][y as usize] = i;
    }
    for i in 0..NUM_ROWS {
        for j in 0..NUM_COLS {
            print!("{:02} ", map[i as usize][j as usize]);
        }
        print!("\n");
    }
}

fn visitable_children(squares: &HashSet<Pos>, children: &Vec<Pos>) ->usize {
    let mut count: usize = 0;
    for child in children {
        if squares.contains(child) {
            count += 1;
        }
    }
    count
}

pub fn find_tour(
    start_pos: &Pos,
    squares: &mut HashSet<Pos>,
    possible: &HashMap<Pos, Vec<Pos>>,
    tour: &mut Vec<Pos>,
) -> SearchResult {
    if squares.contains(start_pos) {
        // Remove the square from the candidates to search, i.e. visit the square.
        squares.remove(start_pos);

        // If there are no more candidate squares to visit, then we are done.
        if squares.len() == 0 {
            return SearchResult::Found;
        }

        // Else, iterate over the remaining squares accessible to `start_pos` and search for a solution.
        // TODO: order the searches by size of pool of candidates. (i.e. if one square can visit 1
        // square and another can visit 8, try the square with fewer possibilities first)
        if let Some(p) = possible.get(start_pos) {
            let mut visitable: Vec<(Pos, usize)> = Vec::new();
            for square in p {
                if squares.contains(square) {
                    if let Some(children) = possible.get(square) {
                        let count: usize = visitable_children(squares, children);
                        let t = (*square, count);
                        visitable.push(t);
                    }
                }
            }
            // Sort visitable squares by increasing number of visitable children
            visitable.sort_by(|a, b| a.1.cmp(&b.1));

            for (square, _count) in visitable {
                // Add the square to the tour
                tour.push(square);
                match find_tour(
                    &square,
                    squares,
                    possible,
                    tour,
                ) {
                    SearchResult::Found => {
                        return SearchResult::Found;
                    }
                    SearchResult::NotFound => {
                        // Remove the square from the tour
                        let end: usize = tour.len() - 1;
                        tour.remove(end);
                    }
                }
            }
        }
        squares.insert(*start_pos);
    }

    SearchResult::NotFound
}


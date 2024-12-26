use advent::prelude::*;

#[derive(Debug, HasParser)]
enum Cell {
    #[parse(string = "#")]
    Filled,
    #[parse(string = ".")]
    Empty
}

#[part_one]
fn part_one(input: List<Grid<Cell>, SepBy<NewLine>>) -> usize {
    let mut locks: Vec<Vec<usize>> = Vec::new();
    let mut keys: Vec<Vec<usize>> = Vec::new();

    for grid in input {
        let vals = grid.columns().map(|col| col.iter().filter(|cell| matches!(cell, Cell::Filled)).count() - 1).collect();
        if matches!(grid[0][0], Cell::Filled) {
            locks.push(vals);
        } else {
            keys.push(vals);
        }
    }  

    let mut count = 0;
    
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut fits = true;
            for (l, k) in lock.iter().zip(key) {
                if l + k > 5 {
                    fits = false;
                }
            }
            if fits {
                count += 1;
            }
        }
    }

    count
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 2691);
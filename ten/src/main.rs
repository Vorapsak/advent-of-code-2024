use advent::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Digit(u8);

impl HasParser for Digit {
    #[into_parser]
    fn parser() -> _ {
        digit().map(|d| Self(d.to_string().parse().unwrap()))
    }
}

#[part_one]
fn part_one(grid: Grid<Digit>) -> usize {
    let mut paths = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let Digit(val) = grid[y][x];
            if val == 0 {
                paths += calculate_paths(&grid, x, y);
            }
        }
    }
    paths
}

fn calculate_paths(grid: &Grid<Digit>, x: usize, y: usize) -> usize {
    let mut paths = HashSet::new();
    let mut to_visit = vec![(x,y)];
    while !to_visit.is_empty() {
        if let Some((x, y)) = to_visit.pop(){
            let Digit(val) = grid[y][x];
            let mut cardinals = vec![];
            if x < grid.width()-1 {
                cardinals.push((x+1, y));
            }
            if y < grid.height()-1 {
                cardinals.push((x, y+1));
            }
            if x > 0 {
                cardinals.push((x-1, y));
            }
            if y > 0 {
                cardinals.push((x, y-1));
            }
            for (x,y) in cardinals {
                let Digit(candidate) = grid[y][x];
                if candidate == val + 1 {
                    if candidate == 9 {
                        paths.insert((x,y));
                    } else {
                        to_visit.push((x,y));
                    }
                }
            }
        }
    }
    paths.len()
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
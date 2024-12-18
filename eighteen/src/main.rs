use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = ",")]
struct Coordinate {
    x: usize,
    y: usize
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Empty,
    Corrupted
}

#[part_one]
fn part_one(input: List<Coordinate, TermWith<NewLine>>) -> usize {
    let width = 71;
    let height = 71;
    let mut grid = vec![vec![Cell::Empty; width]; height];

    for coord in input.iter().take(1024) {
        grid[coord.y][coord.x] = Cell::Corrupted;
    }

    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    costs.insert((0,0), 0);
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    to_visit.push((0,0));

    while !to_visit.is_empty() {
        let (x,y) = to_visit.pop().unwrap();
        let xy_cost = costs.get(&(x,y)).unwrap().clone();
        //up
        if x > 0 {
            if matches!(grid[y][x-1], Cell::Empty) {
                if costs.contains_key(&(x-1,y)) {
                    if *costs.get(&(x-1,y)).unwrap() > xy_cost+1 {
                        *costs.entry((x-1,y)).or_default() = xy_cost+1;
                        to_visit.push((x-1,y));
                    }
                } else {
                    costs.insert((x-1,y), xy_cost+1);
                    to_visit.push((x-1,y));
                }
            }
        }
        //down
        if x < width-1 {
            if matches!(grid[y][x+1], Cell::Empty) {
                if costs.contains_key(&(x+1,y)) {
                    if *costs.get(&(x+1,y)).unwrap() > xy_cost+1 {
                        *costs.entry((x+1,y)).or_default() = xy_cost+1;
                        to_visit.push((x+1,y));
                    }
                } else {
                    costs.insert((x+1,y), xy_cost+1);
                    to_visit.push((x+1,y));
                }
            }
        }
        //up
        if y > 0 {
            if matches!(grid[y-1][x], Cell::Empty) {
                if costs.contains_key(&(x,y-1)) {
                    if *costs.get(&(x,y-1)).unwrap() > xy_cost+1 {
                        *costs.entry((x,y-1)).or_default() = xy_cost+1;
                        to_visit.push((x,y-1));
                    }
                } else {
                    costs.insert((x,y-1), xy_cost+1);
                    to_visit.push((x,y-1));
                }
            }
        }
        //up
        if y < height-1 {
            if matches!(grid[y+1][x], Cell::Empty) {
                if costs.contains_key(&(x,y+1)) {
                    if *costs.get(&(x,y+1)).unwrap() > xy_cost+1 {
                        *costs.entry((x,y+1)).or_default() = xy_cost+1;
                        to_visit.push((x,y+1));
                    }
                } else {
                    costs.insert((x,y+1), xy_cost+1);
                    to_visit.push((x,y+1));
                }
            }
        }
    }



    //dbg!(&costs);
    *costs.get(&(width-1, height-1)).unwrap()
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 248);
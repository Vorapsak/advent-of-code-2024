use advent::prelude::*;

#[derive(HasParser, Debug)]
enum Cell {
    #[parse(string = "#")]
    Wall,
    #[parse(string = ".")]
    Empty,
    #[parse(string = "S")]
    Start,
    #[parse(string = "E")]
    End
}

fn traverse(grid: &Grid<Cell>, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> usize {
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    costs.insert((start_x, start_y), 0);
    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    to_visit.push((start_x, start_y));


    while !to_visit.is_empty() {
        let (x,y) = to_visit.pop().unwrap();
        let xy_cost = costs.get(&(x,y)).unwrap().clone();
        //up
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
        //down
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
        //up
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
        //up
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

    *costs.get(&(end_x, end_y)).unwrap()
    
}

#[part_one]
fn part_one(mut grid: Grid<Cell>) -> usize {
    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;

    for x in 1..grid.width()-1 {
        for y in 1..grid.height()-1 {
            if matches!(grid[y][x], Cell::Start) {
                start_x = x;
                start_y = y;
            } else if matches!(grid[y][x], Cell::End) {
                end_x = x;
                end_y = y;
            }
        }
    }

    dbg!(start_x, start_y, end_x, end_y);

    grid[start_y][start_x] = Cell::Empty;
    grid[end_y][end_x] = Cell::Empty;

    let default_cost = traverse(&grid, start_x, start_y, end_x, end_y);
    let mut costs: HashMap< usize, usize> = HashMap::new();

    for x in 1..grid.width()-1 {
        for y in 1..grid.height()-1 {
            if matches!(grid[y][x], Cell::Wall) {
                grid[y][x] = Cell::Empty;
                let cost = traverse(&grid, start_x, start_y, end_x, end_y);
                if cost < default_cost {
                    *costs.entry(default_cost - cost).or_default() += 1;
                }
                grid[y][x] = Cell::Wall;
            }
        }
    }
    dbg!(&costs);
    costs.iter().map(|(&savings, &count)| if savings > 99 {count} else {0}).sum()
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
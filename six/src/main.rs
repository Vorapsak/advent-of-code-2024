use advent::prelude::*;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Facing {
    North,
    South,
    East,
    West
}


#[part_one]
fn part_one(input: String) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut x = 0;
    let mut y = 0;
    let mut dir = Facing::North;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                x = j;
                y = i;
                
            }
        }
    }

    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert((x, y));

    while x < grid.len() && y < grid[0].len() {
        let next_y = match dir {
            Facing::North => if y > 0 {y - 1} else {break},
            Facing::South => if y < grid.len()-1 {y + 1} else {break},
            _ => y
        };

        let next_x = match dir {
            Facing::West => if x > 0 {x - 1} else {break},
            Facing::East => if x < grid[0].len()-1 {x + 1} else {break},
            _ => x
        };

        if grid[next_y][next_x] == '#' {
            dir = match dir {
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North
            };
        } else {
            x = next_x;
            y = next_y;
        }

        positions.insert((x, y));
    }



    positions.len()
}

#[derive(HasParser, Debug, Clone)]
enum Cell {
    #[parse(string = ".")]
    Empty,
    #[parse(string = "#")]
    Obstacle,
    #[parse(string = "^")]
    Guard
}

#[part_two]
fn part_two(input: Grid<Cell>) -> usize {
        let mut start_x = 0;
        let mut start_y = 0;

        for i in 0..input.width() {
            for j in 0..input.height() {
                if matches!(input[j][i],Cell::Guard) {
                    start_x = i;
                    start_y = j;
                }
            }
        }
    
        let mut cycles = 0;

        let mut grids_to_test = input.clone();
        
        for i in 0..grids_to_test.width() {
            for j in 0..grids_to_test.height() {
                if matches!(grids_to_test[j][i], Cell::Empty) {
                    grids_to_test[j][i] = Cell::Obstacle;
                    if check_cycle(&grids_to_test, start_x, start_y) {
                        cycles += 1;
                    }
                    grids_to_test[j][i] = Cell::Empty;
                }
            }
        }


    cycles
}

fn check_cycle(grid: &Grid<Cell>, start_x: usize, start_y: usize) -> bool {
    let mut positions: HashSet<(usize, usize, Facing)> = HashSet::new();
    let mut x = start_x;
    let mut y = start_y;
    let mut dir = Facing::North;
    positions.insert((x, y, dir));

    while x < grid.width() && y < grid.height() {
        let next_y = match dir {
            Facing::North => if y > 0 {y - 1} else {break},
            Facing::South => if y < grid.width()-1 {y + 1} else {break},
            _ => y
        };

        let next_x = match dir {
            Facing::West => if x > 0 {x - 1} else {break},
            Facing::East => if x < grid[0].len()-1 {x + 1} else {break},
            _ => x
        };

        if matches!(grid[next_y][next_x], Cell::Obstacle) {
            dir = match dir {
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North
            };
        } else {
            x = next_x;
            y = next_y;
        }
        if positions.contains(&(x, y, dir)) {
            return true;
        }
        positions.insert((x, y, dir));
    }

    false
}

harness!(part_1: 4964);
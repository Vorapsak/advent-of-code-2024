use advent::prelude::*;

#[derive(Debug)]
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

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 4964);
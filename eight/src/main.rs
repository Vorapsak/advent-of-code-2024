use advent::prelude::*;

#[part_one]
fn part_one(input: String) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennae: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let max_y = grid.len();
    let max_x = grid[0].len();

    for (j, row) in grid.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c != '.' {
                antennae.entry(*c).or_default().push((i,j));
            }
        }
    }

    let mut nodes: HashSet<(usize, usize)> = HashSet::new();

    for k in antennae.keys() {
        let v = antennae.get(k).unwrap();
        for (start_x, start_y) in v {
            for (other_x, other_y) in v {
                if (start_x, start_y) != (other_x, other_y) {
                    if start_y*2 < *other_y || start_x*2 < *other_x {continue;}
                    let candidate_x = start_x + start_x - other_x;
                    let candidate_y = start_y + start_y - other_y;
                    if candidate_y < max_y && candidate_x < max_x {
                        nodes.insert((candidate_y, candidate_x));
                    }
                }
            }
        }
    }

    nodes.len()
}

#[part_two]
fn part_two(input: String) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennae: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let max_y = grid.len().try_into().unwrap();
    let max_x = grid[0].len().try_into().unwrap();

    for (j, row) in grid.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c != '.' {
                antennae.entry(*c).or_default().push((i.try_into().unwrap(),j.try_into().unwrap()));
            }
        }
    }

    let mut nodes: HashSet<(isize, isize)> = HashSet::new();

    for k in antennae.keys() {
        let v = antennae.get(k).unwrap();
        for (start_x, start_y) in v {
            for (other_x, other_y) in v {
                if (start_x, start_y) != (other_x, other_y) {
                    let dx = start_x - other_x;
                    let dy = start_y - other_y;
                    let mut x = *start_x;
                    let mut y = *start_y;
                    while y >= 0 && y < max_y && x >= 0 && x < max_x {
                        nodes.insert((y, x));
                        x += dx;
                        y += dy;
                    }
                }
            }
        }
    }

    nodes.len()
}

harness!(part_1: 423, part_2: 1287);
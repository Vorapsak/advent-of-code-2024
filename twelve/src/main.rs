use advent::prelude::*;

#[part_one]
fn part_one(input: Grid<char>) -> usize {

    let mut regions: HashMap<char, Vec<Vec<(usize, usize)>>> = HashMap::new();

    for y in 0..input.height() {
        for x in 0..input.width() {
            let c = input[y][x];
            if regions.contains_key(&c) {
                if regions.get_mut(&c).unwrap().iter_mut().all(|region| !region.contains(&(x,y))) {
                        regions.get_mut(&c).unwrap().push(connected_same(&input, x, y));
                }
            } else {
                regions.insert(c, vec![connected_same(&input, x, y)]);
            }
        }
    }

    regions.values().map(|char_regions| char_regions.iter().map(|region| cost(region, &input)).sum::<usize>()).sum()
}

fn connected_same(grid: &Grid<char>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut to_check = vec![(x,y)];
    let id = grid[y][x];
    let mut plots= vec![(x,y)];
    let mut already_seen: HashSet<(usize, usize)> = HashSet::new();
    already_seen.insert((x,y));

    while !to_check.is_empty() {
        let (x, y) = to_check.pop().unwrap();
        let mut neighbor_candidates = Vec::new();
        if x < grid.width()-1 { neighbor_candidates.push((x+1,y)); }
        if y < grid.height()-1 { neighbor_candidates.push((x,y+1));}
        if x > 0 { neighbor_candidates.push((x-1,y)); }
        if y > 0 { neighbor_candidates.push((x, y-1)); }
        for (cx, cy) in neighbor_candidates {
            if grid[cy][cx] == id && !already_seen.contains(&(cx,cy)){
                to_check.push((cx,cy));
                plots.push((cx,cy));
                already_seen.insert((cx,cy));
            }
        }
    }
    
    plots
}

fn cost(region: &Vec<(usize, usize)>, grid: &Grid<char>) -> usize {
    let mut perimeter = 0;
    let mut area = 0;
    for (x,y) in region {
        let mut neighbor_candidates = Vec::new();
        if *x < grid.width()-1 { neighbor_candidates.push((x+1,*y)); }
        if *y < grid.height()-1 { neighbor_candidates.push((*x,y+1));}
        if *x > 0 { neighbor_candidates.push((x-1,*y)); }
        if *y > 0 { neighbor_candidates.push((*x, y-1)); }
        let mut walls = 4;
        for tup in neighbor_candidates {
            if region.contains(&tup) {
                walls -= 1;
            }
        }
        perimeter += walls;
        area += 1
    }
    
    area * perimeter
}

#[part_two]
fn part_two(input: Grid<char>) -> usize {
    let mut regions: HashMap<char, Vec<Vec<(usize, usize)>>> = HashMap::new();

    for y in 0..input.height() {
        for x in 0..input.width() {
            let c = input[y][x];
            if regions.contains_key(&c) {
                if regions.get_mut(&c).unwrap().iter_mut().all(|region| !region.contains(&(x,y))) {
                        regions.get_mut(&c).unwrap().push(connected_same(&input, x, y));
                }
            } else {
                regions.insert(c, vec![connected_same(&input, x, y)]);
            }
        }
    }

    regions.values().map(|char_regions| char_regions.iter().map(|region| cost2(region, &input)).sum::<usize>()).sum()

}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn rotate(&mut self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

fn cost2(region: &Vec<(usize, usize)>, grid: &Grid<char>) -> usize {
    


    region.len()
}

harness!(part_1: 1485656);
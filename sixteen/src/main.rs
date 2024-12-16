use advent::prelude::*;

#[derive(HasParser, Debug, Clone, Copy)]
enum Cell {
    #[parse(string = "#")]
    Wall,
    #[parse(string = ".")]
    Empty,
    #[parse(string = "E")]
    End,
    #[parse(string = "S")]
    Start
}

#[derive(Debug, Default, Copy, Clone)]
enum Facing {
    North,
    South,
    #[default]
    East,
    West
}

#[part_one]
fn part_one(map: Grid<Cell>) -> usize {
    let start_pos: (usize, usize) = (1, map.height()-2);
    let end_pos: (usize, usize) = (map.width()-2, 1);
    let mut to_check = vec![start_pos];
    let mut scores: HashMap<(usize, usize), (usize, Facing)> = HashMap::new();
    scores.insert(start_pos, (0, Facing::East));


    while !to_check.is_empty() {
        let (x, y) = to_check.pop().unwrap();
        let (cur_score, entered_direction) = scores.get(&(x,y)).unwrap().clone();

        if !matches!(map[y][x-1], Cell::Wall) {
            let cost = if matches!(entered_direction, Facing::West) {
                cur_score + 1
            } else {
                cur_score+1001
            };
            let dest = scores.entry((x-1,y)).or_default();
            if dest.0 == 0 || dest.0 > cost {
                *dest = (cost, Facing::West);
                to_check.push((x-1,y));
            }
        }
        if !matches!(map[y][x+1], Cell::Wall) {
            let cost = if matches!(entered_direction, Facing::East) {
                cur_score + 1
            } else {
                cur_score+1001
            };
            let dest = scores.entry((x+1,y)).or_default();
            if dest.0 == 0 || dest.0 > cost {
                *dest = (cost, Facing::East);
                to_check.push((x+1,y));
            }
        }
        if !matches!(map[y-1][x], Cell::Wall) {
            let cost = if matches!(entered_direction, Facing::North) {
                cur_score + 1
            } else {
                cur_score+1001
            };
            let dest = scores.entry((x,y-1)).or_default();
            if dest.0 == 0 || dest.0 > cost {
                *dest = (cost, Facing::North);
                to_check.push((x,y-1));
            }
        }
        if !matches!(map[y+1][x], Cell::Wall) {
            let cost = if matches!(entered_direction, Facing::South) {
                cur_score + 1
            } else {
                cur_score+1001
            };
            let dest = scores.entry((x,y+1)).or_default();
            if dest.0 == 0 || dest.0 > cost {
                *dest = (cost, Facing::South);
                to_check.push((x,y+1));
            }
        }
    }
    
    scores.get(&(end_pos.0,end_pos.1)).unwrap().0
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 83432);
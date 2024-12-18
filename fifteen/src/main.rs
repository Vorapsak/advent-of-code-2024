use advent::prelude::*;

#[derive(HasParser, Debug)]
enum Cell {
    #[parse(string = "#")]
    Wall,
    #[parse(string = ".")]
    Empty,
    #[parse(string = "O")]
    Box,
    #[parse(string = "@")]
    Robot
}

#[derive(HasParser, Debug)]
enum Direction {
    #[parse(string = "<")]
    Left,
    #[parse(string = ">")]
    Right,
    #[parse(string = "^")]
    Up,
    #[parse(string = "v")]
    Down
}

#[derive(HasParser, Debug)]
#[parse(sep_by = "\n")]
struct Input { 
    map: Grid<Cell>,
    moves: List<Vec<Direction>,TermWith<NewLine>>,
}

#[part_one]
fn part_one(mut input: Input) -> usize {

    let mut start_x = 0;
    let mut start_y = 0;

    for x in 1..input.map.width()-1 {
        for y in 1..input.map.height()-1 {
            if matches!(input.map[y][x], Cell::Robot) {
                start_x = x;
                start_y = y;
            }
        }
    }

    input.map[start_y][start_x] = Cell::Empty;


    let mut x = start_x;
    let mut y = start_y;

    for movement in input.moves.into_iter().flatten() {
        let (target_x, target_y) = match movement {
            Direction::Left => (x-1, y),
            Direction::Right => (x+1, y),
            Direction::Up => (x, y-1),
            Direction::Down => (x, y+1),
        };

        

        if matches!(input.map[target_y][target_x], Cell::Empty) {
            x = target_x;
            y = target_y;
        } else if matches!(input.map[target_y][target_x], Cell::Wall){
            continue;
        } else {
            let (eval_x, eval_y) = find_next_wall_or_empty(&input.map, target_x, target_y, movement);
            if matches!(input.map[eval_y][eval_x], Cell::Wall) {
                continue;
            }
            input.map[eval_y][eval_x] = Cell::Box;
            input.map[target_y][target_x] = Cell::Empty;
            x = target_x;
            y = target_y
        }
    }

    input.map.rows().enumerate()
        .map(|(x, row)| 
            row.iter().enumerate().map(|(y, cell)| if matches!(cell, Cell::Box) { y + 100*x } else { 0 })
        .sum::<usize>())
    .sum()

}

fn find_next_wall_or_empty(map: &Grid<Cell>, x: usize, y: usize, dir: Direction) -> (usize, usize) {
    let mut x = x;
    let mut y = y;
    while matches!(map[y][x], Cell::Box) {
        (x, y) = match dir {
            Direction::Left => (x-1, y),
            Direction::Right => (x+1, y),
            Direction::Up => (x, y-1),
            Direction::Down => (x, y+1),
        };
    }
    (x,y)
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
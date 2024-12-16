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
fn part_one(input: Input) -> &'static str {
    dbg!(&input);

    for movement in input.moves.into_iter().flatten() {
        dbg!(&movement);
    }
    
    "incomplete"
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
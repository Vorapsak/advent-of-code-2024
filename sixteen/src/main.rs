use advent::prelude::*;

#[derive(HasParser, Debug)]
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

#[part_one]
fn part_one(map: Grid<Cell>) -> &'static str {
    dbg!(&map);
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for x in 0..map.width() {
        for y in 0..map.height() {
            if matches!(map[y][x], Cell::Start) {
                start_pos = (x,y);
            } else if matches!(map[y][x], Cell::End) {
                end_pos = (x,y);
            }
        }
    }
    dbg!(start_pos, end_pos);
    "incomplete"
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
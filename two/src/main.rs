use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = " ")]
struct InputLine {
    nums: List<i32, SepBy<Space>>
}

#[part_one]
fn part_one(input: List<InputLine, TermWith<NewLine>>) -> usize {
    input.into_iter().map(|line| is_valid(line)).filter(|v| *v).count()
}

fn is_valid(line: InputLine) -> bool {
    let mut current = &line.nums[0];
    let mut dir = 1;
    if line.nums[0] > line.nums[1] { dir = -1;}
    for val in &line.nums[1..]{
        if val.abs_diff(*current) > 3 || val == current {return false}
        if (dir > 0 && current > val) || (dir < 0 && current < val) {return false}
        current = val;
    }
    true
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
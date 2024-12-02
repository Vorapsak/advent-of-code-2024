use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = " ")]
struct InputLine {
    nums: List<i32, SepBy<Space>>
}

#[part_one]
fn part_one(input: List<InputLine, TermWith<NewLine>>) -> usize {
    input.into_iter().filter(|line| is_valid(&line.nums)).count()
}

fn is_valid(line: &List<i32, SepBy<Space>>) -> bool {
    let mut current = &line[0];
    let descending = line[0] > line[1];
    for val in &line[1..]{
        if val.abs_diff(*current) > 3 || val == current {return false}
        if (!descending && current > val) || (descending && current < val) {return false}
        current = val;
    }
    true
}

#[part_two]
fn part_two(input: List<InputLine, TermWith<NewLine>>) -> usize {
    for line in input {
        
    }
    0
}

harness!(part_1: 218);
use advent::prelude::*;
use parser::range;

#[derive(Debug, HasParser, Clone)]
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

fn is_valid2(line: Vec<i32>) -> bool {
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
    let mut safe = 0;
    for line in input {
        if is_valid(&line.nums) {
            safe += 1;
            continue;
        }
        // test each removal
        for i in 0 .. line.nums.len() {
            let mut to_test = Vec::new();
            for (j, num) in line.clone().nums.into_iter().enumerate() {
                if j == i {continue;}
                to_test.push(num);
            }
            if is_valid2(to_test) {
                safe += 1;
                break;
            }
        }
    }
    safe
}

harness!(part_1: 218, part_2: 290);
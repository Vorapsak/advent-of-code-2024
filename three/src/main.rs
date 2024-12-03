use advent::prelude::*;

#[part_one]
fn part_one(input: String) -> u32 {
    //let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
    let mut counter = 0;
    let mut i = 0;
    let input: Vec<_> = input.chars().collect();

    while i < input.len() {
        if input[i] != 'm' {
            i += 1;
            continue;
        }
        i += 1;
        if input[i] != 'u' {continue;}
        i += 1;
        if input[i] != 'l' {continue;}
        i += 1;
        if input[i] != '(' {continue;}
        i += 1;
        if let Some((arg1, delta_i)) = parse_up_to_three(&input, i) {
            i += delta_i;
            if input[i] != ',' {
                continue;
            }
            i += 1;
            if let Some((arg2, delta_i)) = parse_up_to_three(&input, i) {
                i += delta_i;
                if input[i] != ')' {
                    continue;
                }
                i += 1;
                counter += arg1 * arg2;
            }
        } else {
            continue;
        }
    }
    counter
}

fn parse_up_to_three(input: &Vec<char>, i: usize) -> Option<(u32,usize)> {
    if let Ok(arg) = input[i..i+3].iter().collect::<String>().parse::<u32>() {
        Some((arg,3))
    } else if let Ok(arg) = input[i..i+2].iter().collect::<String>().parse::<u32>(){
        Some((arg,2))
    } else if let Some(arg) = input[i].to_digit(10) {
        Some((arg,1))
    } else {
        None
    }
}

#[part_two]
fn part_two(input: String) -> &'static str {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
    let mut counter = 0;
    let mut i = 0;
    let input: Vec<_> = input.chars().collect();
    let mut execute = true;

    while i < input.len() {
        if input[i] != 'm' && input[i] != 'd' {
            i += 1;
            continue;
        }
        i += 1;
        if input[i] != 'u' {continue;}
        i += 1;
        if input[i] != 'l' {continue;}
        i += 1;
        if input[i] != '(' {continue;}
        i += 1;
        if let Some((arg1, delta_i)) = parse_up_to_three(&input, i) {
            i += delta_i;
            if input[i] != ',' {
                continue;
            }
            i += 1;
            if let Some((arg2, delta_i)) = parse_up_to_three(&input, i) {
                i += delta_i;
                if input[i] != ')' {
                    continue;
                }
                i += 1;
                if execute {
                    counter += arg1 * arg2;
                }
                dbg!(counter);
            }
        } else {
            dbg!(input[i]);
            continue;
        }
    }
counter
}

harness!(part_1: 156388521);
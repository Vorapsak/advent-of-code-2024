use advent::prelude::*;

#[part_one]
fn part_one(input: String) -> u32 {
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
fn part_two(input: String) -> u32 {
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
        if input[i] == 'o' {
            i += 1;
            if input[i] == '(' && input[i+1] == ')' {
                execute = true;
                i += 2;
            } else if input[i] == 'n' && input[i+1] == '\'' && input[i+2] == 't' && input[i+3] == '(' && input[i+4] == ')' {
                execute = false;
                i += 5;
            }
            continue;
        }
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
            }
        } else {
            continue;
        }
    }
counter
}

harness!(part_1: 156388521, part_2: 75920122);
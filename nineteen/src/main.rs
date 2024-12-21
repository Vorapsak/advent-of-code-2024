use advent::prelude::*;
use regex::Regex;

#[part_one]
fn part_one(input: String) -> usize {
    let towels: String = input.lines().take(1).collect();
    let targets: Vec<&str> = input.lines().skip(2).collect();
    let mut satisfiable = 0;
    let towels = towels.split(", ").collect::<Vec<_>>();

    let re = Regex::new(&("^(".to_owned() + &towels.join("|") + ")+$")).unwrap();

     for pattern in targets {
        if re.is_match(pattern) {
            satisfiable += 1;
        }
    } 

    satisfiable
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 300);
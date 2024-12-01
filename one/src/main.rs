use advent::prelude::*;
use std::iter::zip;

#[part_one]
fn part_one(input: String) -> u64 {
    let vals: Vec<_> = input.lines().into_iter().map(|line| line.split_once("   ").unwrap()).collect();
    let mut lefts: Vec<u64> = Vec::new();
    let mut rights: Vec<u64> = Vec::new();
    for (l, r) in vals {
        lefts.push(l.parse().unwrap());
        rights.push(r.parse().unwrap());
    }
    lefts.sort();
    rights.sort();
    let mut total_diff: u64 = 0;
    for (l, r) in zip(lefts, rights) {
        total_diff += l.abs_diff(r);
    }
    total_diff
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
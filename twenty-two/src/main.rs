use advent::prelude::*;

#[part_one]
fn part_one(input: List<u64, TermWith<NewLine>>) -> u64 {
    dbg!(&input);
    let mut total = 0;
    for seed in input {
        let mut secret = seed;
        for _ in 0..2000 {
            let t = secret * 64;
            secret = secret ^ t;
            secret %= 16777216;
            let t = secret / 32;
            secret = secret ^ t;
            secret %= 16777216;
            let t = secret * 2048;
            secret = secret ^ t;
            secret %= 16777216;
        }
        total += secret;
    }

    total
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 20506453102);
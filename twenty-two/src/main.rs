use advent::prelude::*;

#[part_one]
fn part_one(input: List<u64, TermWith<NewLine>>) -> u64 {
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
fn part_two(input: List<i64, TermWith<NewLine>>) -> i64 {
    let mut sequences: HashMap<(i64, i64, i64, i64), usize> = HashMap::new();

    for &seed in input.iter() {
        let mut seed_seqs: HashSet<(i64, i64, i64, i64)> = HashSet::new();
        let mut secret = seed;
        let t = iterate(secret);
        let mut four_ago = (t%10) - (secret%10);
        secret = t;
        let t = iterate(secret);
        let mut three_ago = (t%10) - (secret%10);
        secret = t;
        let t = iterate(secret);
        let mut two_ago = (t%10) - (secret%10);
        secret = t;
        let t = iterate(secret);
        let mut one_ago = (t%10) - (secret%10);
        secret = t;
        sequences.insert((four_ago, three_ago, two_ago, one_ago), 1);
        for _ in 4..2000 {
            let next = iterate(secret);
            four_ago = three_ago;
            three_ago = two_ago;
            two_ago = one_ago;
            one_ago = (next%10) - (secret%10);
            secret = next;
            seed_seqs.insert((four_ago, three_ago, two_ago, one_ago));
        }

        for seq in seed_seqs {
            *sequences.entry(seq).or_default() += 1;
        }
    }

    let mut max_bananas = 0;

    for (sequence, count) in sequences {
        if count < 370 { continue;}
        let next = calculate_profits(sequence, &input);
        if next > max_bananas {
            max_bananas = next;
        }
    }

    max_bananas
}

fn calculate_profits (sequence: (i64, i64, i64, i64), input: &List<i64, TermWith<NewLine>>) -> i64 {
    let mut profit = 0;
    for &seed in input {
        
        let mut secret = seed;
        let t = iterate(secret);
        let mut four_ago = (t%10) - (secret%10);
        secret = t;
        let t = iterate(secret);
        let mut three_ago = (t%10) - (secret%10);
        secret = t;
        let t = iterate(secret);
        let mut two_ago = (t%10) - (secret%10);
        secret = t;
        let t = iterate(secret);
        let mut one_ago = (t%10) - (secret%10);
        secret = t;

        for _ in 4..2000 {
            if (four_ago, three_ago, two_ago, one_ago) == sequence {profit += secret%10; break;}
            let next = iterate(secret);
            four_ago = three_ago;
            three_ago = two_ago;
            two_ago = one_ago;
            one_ago = (next%10) - (secret%10);
            secret = next;
        }
    }
    profit
}

fn iterate(mut secret: i64) -> i64 {
    let t = secret * 64;
    secret = secret ^ t;
    secret %= 16777216;
    let t = secret / 32;
    secret = secret ^ t;
    secret %= 16777216;
    let t = secret * 2048;
    secret = secret ^ t;
    secret %= 16777216;
    secret
}

harness!(part_1: 20506453102, part_2: 2423);
use advent::prelude::*;

#[part_one]
fn part_one(input: List<usize, SepBy<Space>>) -> usize {
    let mut current_stones: HashMap<usize, usize> = input.into_iter().map(|val| (val, 1)).collect();

    for _ in 0..25 {
        current_stones = evolve(current_stones);
    }
    current_stones.values().sum()
}

fn evolve(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut next = HashMap::new();

    for (label, count) in stones {
        if (label) == 0 {
            *next.entry(1).or_default() += count;
        } else if label.to_string().len() % 2 == 0 {
            let stonestr = label.to_string();
            let (p1, p2) = stonestr.split_at(stonestr.len()/2);
            let p1 = p1.parse().unwrap();
            let p2 = p2.parse().unwrap();
            *next.entry(p1).or_default() += count;
            *next.entry(p2).or_default() += count;
        } else {
            *next.entry(label * 2024).or_default() += count;
        }
    }

    next
}

#[part_two]
fn part_two(input: List<usize, SepBy<Space>>) -> usize {
    let mut current_stones: HashMap<usize, usize> = input.into_iter().map(|val| (val, 1)).collect();

    for _ in 0..75 {
        current_stones = evolve(current_stones);
    }

    current_stones.values().sum()

}

harness!(part_1: 203609, part_2: 240954878211138);
use advent::prelude::*;

#[part_one]
fn part_one(input: List<usize, SepBy<Space>>) -> usize {
    let mut current_stones: Vec<usize> = input.into_iter().collect();

    for _ in 0..25 {
        current_stones = evolve(current_stones);
    }

    current_stones.len()
}

fn evolve(stones: Vec<usize>) -> Vec<usize> {
    let mut next = Vec::new();

    for stone in stones {
        if stone == 0 {
            next.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            let stonestr = stone.to_string();
            let (p1, p2) = stonestr.split_at(stonestr.len()/2);
            next.push(p1.parse().unwrap());
            next.push(p2.parse().unwrap());
        } else {
            next.push(stone * 2024);
        }
    }


    next
}

#[part_two]
fn part_two(input: List<usize, SepBy<Space>>) -> usize {
    let mut current_stones: Vec<usize> = input.into_iter().collect();

    for _ in 0..25 {
        current_stones = evolve(current_stones);
    }

    current_stones.len()

}

harness!(part_1: 203609);
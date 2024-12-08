use advent::prelude::*;

#[derive(Debug, HasParser)]
struct InputLine {
    #[parse(after=":")]
    total: usize,
    nums: List<usize, SepBy<Space>>
}

impl InputLine {
    fn satisfiable(&self) -> bool {
        let mut states = Vec::new();
        for (idx, num) in self.nums.iter().enumerate() {
            if states.is_empty() {states.push(*num)} else {
                let mut next_states = Vec::new();
                while !states.is_empty() {
                    let old = states.pop().unwrap();
                    if old + num <= self.total {next_states.push(old + num);}
                    if old * num <= self.total {next_states.push(old * num);}
                }
                states = next_states;
            }
        }
        states.contains(&self.total)
    }
}

#[part_one]
fn part_one(input: List<InputLine, SepBy<NewLine>>) -> usize {
    input.iter().filter(|&line| line.satisfiable()).map(|line| line.total).sum()
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 303766880536);
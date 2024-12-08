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
        for num in self.nums.iter() {
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

    fn satisfiable2(&self) -> bool {
        let mut states = Vec::new();
        for num in self.nums.iter(){
            if states.is_empty() {states.push(*num)} else {
                let mut next_states = Vec::new();
                while !states.is_empty() {
                    let old = states.pop().unwrap();
                    if old + num <= self.total {next_states.push(old + num);}
                    if old * num <= self.total {next_states.push(old * num);}
                    let n: usize = format!("{}{}", old, num).parse().unwrap();
                    if n <= self.total {next_states.push(n);}
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
fn part_two(input: List<InputLine, SepBy<NewLine>>) -> usize {
    input.iter().filter(|&line| line.satisfiable2()).map(|line| line.total).sum()
}

harness!(part_1: 303766880536, part_2: 337041851384440);
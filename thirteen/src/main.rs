use advent::prelude::*;

#[derive(HasParser, Debug)]
#[parse(sep_by = "\n")]
struct Machine {
    #[parse(before = "Button A: ")]
    button_a: Button,
    #[parse(before = "Button B: ")]
    button_b: Button,
    prize: Prize,
}

#[derive(Debug, HasParser)]
#[parse(before = "X+", sep_by = ", Y+")]
struct Button {
    x: u32,
    y: u32
}

#[derive(Debug, HasParser)]
#[parse(before = "Prize: X=", sep_by = ", Y=", after = "\n")]
struct Prize {
    x: u32,
    y: u32
}

impl Machine {
    fn cheapest_price(self: &Self) -> u32 {
        let mut plays: Vec<(u32, u32)> = Vec::new();
        for a_presses in (0..100).rev() {
            let a_x_dist = self.button_a.x * a_presses;
            let a_y_dist = self.button_a.y * a_presses;
            if a_x_dist > self.prize.x {continue;}
            if a_y_dist > self.prize.y {continue;}
            let x_left = self.prize.x - a_x_dist;
            let y_left = self.prize.y - a_y_dist;
            if x_left % self.button_b.x != 0 {continue;}
            if y_left % self.button_b.y != 0 {continue;}
            let b_presses = x_left / self.button_b.x;
            if b_presses != y_left / self.button_b.y {continue;}
            plays.push((a_presses, b_presses));
        }

        if plays.len() == 0 { 0 } else { plays.iter().map(|(x,y)| x*3 + y).min().unwrap() }
    }

    fn cheapest_two(self: &Self) -> usize {
        0
    }
}

#[part_one]
fn part_one(machines: List<Machine, SepBy<NewLine>>) -> u32 {
    machines.iter().map(|machine| machine.cheapest_price()).sum()
}

#[part_two]
fn part_two(machines: List<Machine, SepBy<NewLine>>) -> usize {
    machines.iter().map(|machine| machine.cheapest_two()).sum()
}

harness!(part_1: 29438);
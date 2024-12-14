use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = ",")]
struct Pos2d {
    x: i32,
    y: i32
}

#[derive(Debug, HasParser)]
#[parse(before = "p=", sep_by = " v=")]
struct Robot {
    position: Pos2d,
    velocity: Pos2d
}

impl Robot {
    fn advance(self: &mut Self, width: i32, height: i32, num_iters: i32) {
        let x_movement = self.velocity.x * num_iters;
        let y_movement = self.velocity.y * num_iters;
        self.position.x = (self.position.x + x_movement).rem_euclid(width);
        self.position.y = (self.position.y + y_movement).rem_euclid(height);
    }

    fn get_quadrant(self: &Self, xmid: i32, ymid: i32) -> u32 {
        if self.position.x == xmid || self.position.y == ymid {
            5
        } else if self.position.x < xmid && self.position.y < ymid {
            0
        } else if self.position.x < xmid && self.position.y > ymid {
            2
        } else if self.position.x > xmid && self.position.y < ymid {
            1
        } else {
            3
        }
    }
}

#[part_one]
fn part_one(mut robots: List<Robot, TermWith<NewLine>>) -> u32 {
    let width = 101;
    let height = 103;
    let num_iters = 100;


    for robot in robots.iter_mut() {
        robot.advance(width, height, num_iters);
    }

    let xmid = width/2;
    let ymid = height/2;

    let mut quadrant_counts: HashMap<u32, u32> = HashMap::new();
    
    for robot in robots {
        *quadrant_counts.entry(robot.get_quadrant(xmid, ymid)).or_default() += 1;
    }


    quadrant_counts.iter().map(|(&quadrant, &count)| if quadrant < 5 { count } else {1}).fold(1, |acc, count| count * acc)
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 233709840);
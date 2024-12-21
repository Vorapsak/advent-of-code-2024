use advent::prelude::*;

#[part_one]
fn part_one(input: String) -> usize {
    let mut total_cost = 0;
    for target in input.lines() {
        let numeric: usize = target[0..target.len()-1].parse().unwrap();
        let final_robot = convert_numeric_keypad(target);
        let middle_robot = convert_direction_keypad(&final_robot);
        let first_robot = convert_direction_keypad(&middle_robot);
        total_cost += first_robot.len() * numeric;
        dbg!(first_robot.len(), numeric);
    }

    total_cost
}

fn convert_numeric_keypad(sequence: &str) -> String {
    let mut key_presses: Vec<&str> = Vec::new();
    let mut current_position = 'A';

    for target in sequence.chars() {
        key_presses.push(numeric_keypad_transition(current_position, target));
        current_position = target;
    }

    key_presses.into_iter().collect()
}

fn convert_direction_keypad(sequence: &String) -> String {
    let mut key_presses: Vec<&str> = Vec::new();
    let mut current_position = 'A';

    for target in sequence.chars() {
        key_presses.push(direction_keypad_transition(current_position, target));
        current_position = target;
    }

    key_presses.into_iter().collect()
}

fn numeric_keypad_transition(start: char, end: char) -> &'static str {
    match (start, end) {
        ('A', '0') => "<A",
        ('A', '1') => "^<<A",
        ('A', '2') => "^<A",
        ('A', '3') => "^A",
        ('A', '4') => "^^<<A",
        ('A', '5') => "^^<A",
        ('A', '6') => "^^A",
        ('A', '7') => "^^^<<A",
        ('A', '8') => "^^^<A",
        ('A', '9') => "^^^A",
        ('A', 'A') => "A",

        
        ('0', '0') => "A",
        ('0', '1') => "^<A",
        ('0', '2') => "^A",
        ('0', '3') => "^>A",
        ('0', '4') => "^^<A",
        ('0', '5') => "^^A",
        ('0', '6') => "^^>A",
        ('0', '7') => "^^^<A",
        ('0', '8') => "^^^A",
        ('0', '9') => "^^^>A",
        ('0', 'A') => ">A",
        
        ('1', '0') => ">vA",
        ('1', '1') => "A",
        ('1', '2') => ">A",
        ('1', '3') => ">>A",
        ('1', '4') => "^A",
        ('1', '5') => "^>A",
        ('1', '6') => "^>>A",
        ('1', '7') => "^^A",
        ('1', '8') => "^^>A",
        ('1', '9') => "^^>>A",
        ('1', 'A') => ">>vA",
        
        ('2', '0') => "vA",
        ('2', '1') => "<A",
        ('2', '2') => "A",
        ('2', '3') => ">A",
        ('2', '4') => "^<A",
        ('2', '5') => "^A",
        ('2', '6') => "^>A",
        ('2', '7') => "^^<A",
        ('2', '8') => "^^A",
        ('2', '9') => "^^>A",
        ('2', 'A') => ">vA",
        
        ('3', '0') => "<vA",
        ('3', '1') => "<<A",
        ('3', '2') => "<A",
        ('3', '3') => "A",
        ('3', '4') => "^<<A",
        ('3', '5') => "^<A",
        ('3', '6') => "^A",
        ('3', '7') => "^^<<A",
        ('3', '8') => "^^<A",
        ('3', '9') => "^^A",
        ('3', 'A') => "vA",
        
        ('4', '0') => ">vvA",
        ('4', '1') => "vA",
        ('4', '2') => "v>A",
        ('4', '3') => "v>>A",
        ('4', '4') => "A",
        ('4', '5') => ">A",
        ('4', '6') => ">>A",
        ('4', '7') => "^A",
        ('4', '8') => "^>A",
        ('4', '9') => "^>>A",
        ('4', 'A') => ">>vvA",
        
        ('5', '0') => "vvA",
        ('5', '1') => "v<A",
        ('5', '2') => "vA",
        ('5', '3') => "v>A",
        ('5', '4') => "<A",
        ('5', '5') => "A",
        ('5', '6') => ">A",
        ('5', '7') => "^<A",
        ('5', '8') => "^A",
        ('5', '9') => "^>A",
        ('5', 'A') => "vv>A",
        
        ('6', '0') => "<vvA",
        ('6', '1') => "<<vA",
        ('6', '2') => "<vA",
        ('6', '3') => "vA",
        ('6', '4') => "<<A",
        ('6', '5') => "<A",
        ('6', '6') => "A",
        ('6', '7') => "^<<A",
        ('6', '8') => "^<A",
        ('6', '9') => "^A",
        ('6', 'A') => "vvA",
        
        ('7', '0') => ">vvvA",
        ('7', '1') => "vvA",
        ('7', '2') => "vv>A",
        ('7', '3') => "vv>>A",
        ('7', '4') => "vA",
        ('7', '5') => "v>A",
        ('7', '6') => "v>>A",
        ('7', '7') => "A",
        ('7', '8') => ">A",
        ('7', '9') => ">>A",
        ('7', 'A') => ">>vvvA",
        
        ('8', '0') => "vvvA",
        ('8', '1') => "vv<A",
        ('8', '2') => "vvA",
        ('8', '3') => "vv>A",
        ('8', '4') => "v<A",
        ('8', '5') => "vA",
        ('8', '6') => "v>A",
        ('8', '7') => "<A",
        ('8', '8') => "A",
        ('8', '9') => ">A",
        ('8', 'A') => ">vvvA",
    
        ('9', '0') => "<vvvA",
        ('9', '1') => "<<vvA",
        ('9', '2') => "<vv",
        ('9', '3') => "vvA",
        ('9', '4') => "v<<A",
        ('9', '5') => "v<A",
        ('9', '6') => "vA",
        ('9', '7') => "<<A",
        ('9', '8') => "<A",
        ('9', '9') => "A",
        ('9', 'A') => "vvvA",

        (_,_) => panic!(),
    }

}

fn direction_keypad_transition(start: char, end: char) -> &'static str {
    match (start, end) {
        ('A', '^') => "<A",
        ('A', '<') => "v<<A",
        ('A', 'v') => "v<A",
        ('A', '>') => "vA",
        ('A', 'A') => "A",
        
        ('^', '^') => "A",
        ('^', '<') => "v<A",
        ('^', 'v') => "vA",
        ('^', '>') => "v>A",
        ('^', 'A') => ">A",
        
        ('<', '^') => ">^A",
        ('<', '<') => "A",
        ('<', 'v') => ">A",
        ('<', '>') => ">>A",
        ('<', 'A') => ">>^A",
        
        ('v', '^') => "^A",
        ('v', '<') => "<A",
        ('v', 'v') => "A",
        ('v', '>') => ">A",
        ('v', 'A') => ">^A",
        
        ('>', '^') => "^<A",
        ('>', '<') => "<<A",
        ('>', 'v') => "<A",
        ('>', '>') => "A",
        ('>', 'A') => "^A",

        (_,_) => panic!(),
    }
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();
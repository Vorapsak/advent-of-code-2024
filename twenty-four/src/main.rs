use advent::prelude::*;

#[derive(Debug)]
enum Operation {
    AND,
    OR,
    XOR
}

#[derive(Debug)]
struct Gate {
    wire1: String,
    wire2: String,
    operation: Operation,
    output: String,
}


#[part_one]
fn part_one(input: String) -> u64 {
    let (wires, gates) = input.split_once("\n\n").unwrap();
    let mut known_values: HashMap<String, u8> = wires.lines().map(|line| (line.split_once(":").unwrap())).map(|(label, value)| (label.to_string(), value.trim().parse::<u8>().unwrap())).collect();
    let mut gates_to_eval:VecDeque<Gate> = VecDeque::new();

    for gate in gates.lines() {
        if gate.contains("AND") {
            let (wire1, rest) = gate.split_once("AND").unwrap();
            let wire1 = wire1.trim();
            let (wire2, output) = rest.split_once("->").unwrap();
            let wire2 = wire2.trim();
            let output = output.trim();
            gates_to_eval.push_back(Gate {
                wire1: wire1.into(),
                wire2: wire2.into(),
                operation: Operation::AND,
                output: output.into()
            });
        } else if gate.contains("XOR") {
            let (wire1, rest) = gate.split_once("XOR").unwrap();
            let wire1 = wire1.trim();
            let (wire2, output) = rest.split_once("->").unwrap();
            let wire2 = wire2.trim();
            let output = output.trim();
            gates_to_eval.push_back(Gate {
                wire1: wire1.into(),
                wire2: wire2.into(),
                operation: Operation::XOR,
                output: output.into()
            });

        } else if gate.contains("OR") {
            let (wire1, rest) = gate.split_once("OR").unwrap();
            let wire1 = wire1.trim();
            let (wire2, output) = rest.split_once("->").unwrap();
            let wire2 = wire2.trim();
            let output = output.trim();
            gates_to_eval.push_back(Gate {
                wire1: wire1.into(),
                wire2: wire2.into(),
                operation: Operation::OR,
                output: output.into()
            });

        } else {
            panic!();
        }
    }

    while !gates_to_eval.is_empty() {
        let gate = gates_to_eval.pop_front().unwrap();
        if known_values.contains_key(&gate.wire1) && known_values.contains_key(&gate.wire2){
            let in1 = known_values.get(&gate.wire1).unwrap();
            let in2 = known_values.get(&gate.wire2).unwrap();
            match gate.operation {
                Operation::AND => known_values.insert(gate.output, in1 & in2),
                Operation::OR => known_values.insert(gate.output, in1 | in2),
                Operation::XOR => known_values.insert(gate.output, in1 ^ in2),
            };
        } else {
            gates_to_eval.push_back(gate);
        }
    }

    let mut acc = 0;
    for i in 0..=9 {
        if known_values.contains_key(&format!("z0{}", i)) {
            acc += (*known_values.get(&format!("z0{}", i)).unwrap() as u64) * 2_u64.pow(i);
        } else {
            break;
        }
    }

    for i in 10..=99 {
        if known_values.contains_key(&format!("z{}", i)) {
            acc += (*known_values.get(&format!("z{}", i)).unwrap() as u64) * 2_u64.pow(i);
        } else {
            break;
        }
    }

    dbg!(acc);
    
    acc
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 53325321422566);
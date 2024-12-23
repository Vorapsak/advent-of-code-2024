use advent::prelude::*;

#[part_one]
fn part_one(input: String) -> usize {
    let mut adjacents: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut ids: HashSet<&str> = HashSet::new();
    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        adjacents.entry(a).or_default().push(b);
        adjacents.entry(b).or_default().push(a);
        ids.insert(a);
        ids.insert(b);
    }

    let mut trios: HashSet<Vec<&str>> = HashSet::new();

    for computer_id in ids {
        if !computer_id.starts_with('t') {
            continue;
        }
        let neighbors = adjacents.get(computer_id).unwrap();
        for &neighbor in neighbors {
            let cousins = adjacents.get(neighbor).unwrap();
            for &cousin in cousins {
                if adjacents.get(cousin).unwrap().contains(&computer_id) {
                    let mut t = vec![computer_id, cousin, neighbor];
                    t.sort();
                    trios.insert(t);
                }

            }
        }

    }

    trios.len()

}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 1163);
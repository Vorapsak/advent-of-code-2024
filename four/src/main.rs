use advent::prelude::*;

#[part_one]
fn part_one(input: String) -> usize {

    let input = input.lines();

    let mut count = 0;
    let mut verts: Vec<Vec<char>> = vec![Vec::new(); 25000];
    let mut diags1: HashMap<i32, Box<Vec<char>>> = HashMap::new();
    let mut diags2: HashMap<i32, Box<Vec<char>>> = HashMap::new();

    for (i, line) in input.enumerate() {
        //forward
        count += line.split("XMAS").collect::<Vec<_>>().len() - 1;
        //backward
        count += line.chars().rev().collect::<String>().split("XMAS").collect::<Vec<_>>().len() - 1;

        for (j, letter) in line.chars().enumerate() {
            verts[j].push(letter);
            diags1.entry(i as i32 - j as i32).or_default().push(letter);
            diags2.entry((line.len() - j) as i32 - i as i32).or_default().push(letter);
        }
    }

    for line in verts {
        if line.len() == 0 {continue;}
         //down
         count += line.iter().collect::<String>().split("XMAS").collect::<Vec<_>>().len() - 1;
         //up
         count += line.iter().rev().collect::<String>().split("XMAS").collect::<Vec<_>>().len() - 1;
    }
    
    for line in diags1.values() {
         //down
         count += line.iter().collect::<String>().split("XMAS").collect::<Vec<_>>().len() - 1;
         //up
         count += line.iter().rev().collect::<String>().split("XMAS").collect::<Vec<_>>().len() - 1;
    }

    for line in diags2.values() {
         //down
         count += line.iter().collect::<String>().split("XMAS").collect::<Vec<_>>().len() - 1;
         //up
         count += line.iter().rev().collect::<String>().split("XMAS").collect::<Vec<_>>().len() - 1;
    }

    count
}


#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 2464);
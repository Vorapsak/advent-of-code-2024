use core::fmt;

use advent::prelude::*;

struct Filesystem {
    memory: Vec<MemoryBlock>
}

#[derive(Debug)]
enum MemoryBlock {
    File(usize),
    Empty
}

#[derive(Debug)]
enum Mode {
    File,
    Empty
}

impl Filesystem {
    fn from(input: String) -> Self {
        let mut mem = Vec::new();
        let mut current_id = 0;
        let mut mode = Mode::File;

        for c in input.chars() {
            let c = c.to_digit(10).unwrap();
            match mode {
                Mode::File => {for _ in 1..=c {mem.push(MemoryBlock::File(current_id));}; 
                    mode = Mode::Empty;
                    current_id += 1;
                },
                Mode::Empty => {for _ in 1..=c {mem.push(MemoryBlock::Empty);}; 
                    mode = Mode::File;
                }
            }
        }
        

        Filesystem {
            memory: mem
        }
    }
    fn rposition<P: FnMut(&MemoryBlock) -> bool> (&self, predicate: P) -> Option<usize> {
        self.memory.iter().rposition(predicate)
    }
    fn swap(&mut self, a: usize, b: usize) {
        self.memory.swap(a, b);
    }
    fn position<P: FnMut(&MemoryBlock) -> bool> (&self, predicate: P) -> Option<usize> {
        self.memory.iter().position(predicate)
    }

    fn checksum(&self) -> usize {
        self.memory.iter().enumerate().map(|(idx, val)| match val {
            MemoryBlock::File(file_id) => file_id * idx,
            MemoryBlock::Empty => 0
        }).sum()
    }
}

impl fmt::Debug for Filesystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chars = "".to_string();
        for cell in self.memory.iter() {
            match cell {
                MemoryBlock::File(id) => chars.push_str(&id.to_string()),
                MemoryBlock::Empty => chars.push('.'),
            }
        }
        write!(f, "{}", chars)
    }
}

#[part_one]
fn part_one(input: String) -> usize {
    let mut fs = Filesystem::from(input);
    dbg!(&fs);

    loop {
        let file_to_move_pos = fs.rposition(|x| !matches!(x, MemoryBlock::Empty)).unwrap();
        let space_to_fill_pos = fs.position(|x| matches!(x, MemoryBlock::Empty)).unwrap();
        if file_to_move_pos < space_to_fill_pos {
            break;
        }
        fs.swap(file_to_move_pos, space_to_fill_pos);
        //dbg!(&fs);
    }
    
    dbg!(&fs);

    fs.checksum()

}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!(part_1: 6390180901651);
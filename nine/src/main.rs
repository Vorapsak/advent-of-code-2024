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

    loop {
        let file_to_move_pos = fs.rposition(|x| !matches!(x, MemoryBlock::Empty)).unwrap();
        let space_to_fill_pos = fs.position(|x| matches!(x, MemoryBlock::Empty)).unwrap();
        if file_to_move_pos < space_to_fill_pos {
            break;
        }
        fs.swap(file_to_move_pos, space_to_fill_pos);
        //dbg!(&fs);
    }

    fs.checksum()

}

#[derive(Debug, Clone)]
enum FileBlock {
    File {
        start_position: usize,
        size: usize,
        id: usize
    },
    Empty {
        start_position: usize,
        size: usize
    }
}

#[derive(Debug)]
struct Filesystem2 {
    files: Vec<FileBlock>,
    max_id: usize
}

impl Filesystem2 {
    fn from(input: String) -> Self {
        let mut files = Vec::new();
        let mut current_id = 0;
        let mut mode = Mode::File;
        let mut current_position = 0;
        let mut max_id = 0;
    
        for c in input.chars() {
            let c = c.to_digit(10).unwrap() as usize;
            if c == 0 {
                mode = Mode::File;
                continue;}
            match mode {
                Mode::File => {
                    files.push(FileBlock::File{
                        start_position: current_position,
                        size: c,
                        id: current_id
                    });
                    mode = Mode::Empty;
                    max_id = current_id;
                    current_id += 1;
                },
                Mode::Empty => {
                    files.push(FileBlock::Empty {
                        start_position: current_position,
                        size: c
                    }); 
                    mode = Mode::File;
                }
            }
            current_position += c;
        }

        Filesystem2 {
            files: files,
            max_id: max_id
        }
    }

    fn position(&self, target_id: usize) -> usize {
        self.files.iter().position(|x| match x {
            FileBlock::File{id, ..} => *id == target_id,
            _ => false
        }).unwrap()
    }

    fn try_move(&mut self, file_arr_position: usize, empty_arr_location: usize) -> bool {
        match self.files[empty_arr_location] {
            FileBlock::File{ .. } => false,
            FileBlock::Empty{size, start_position, ..} => {
                let empty_size = size;
                let empty_start = start_position;
                let (file_size, file_start) = match self.files[file_arr_position] {
                    FileBlock::Empty { .. } => (0, 0),
                    FileBlock::File {size, start_position, ..} => (size, start_position)
                };
                if file_size == 0 { return false; }
                if file_size > empty_size { return false; }
                if file_start < empty_start { return false; }
                
                let file_s = self.files[file_arr_position].clone();
                self.files.remove(file_arr_position);
                let file_s = match file_s {
                    FileBlock::Empty { .. } => panic!("This should be impossible"),
                    FileBlock::File { id, size, .. } => FileBlock::File { start_position: empty_start, size: size, id: id }
                };
                self.files.remove(empty_arr_location);
                
                self.files.push(file_s);

                if empty_size > file_size {
                    let remaining_space = empty_size - file_size;
                    let new_empty_start = empty_start + file_size;
                    self.files.push(FileBlock::Empty { start_position: new_empty_start, size: remaining_space });
                }

                self.files.push(FileBlock::Empty { start_position: file_start, size: file_size });
                
                self.files.sort_by(|f1, f2| match f1 {
                    FileBlock::Empty { start_position, .. } => start_position,
                    FileBlock::File { start_position, .. } => start_position
                }.cmp(match f2 {
                    FileBlock::Empty { start_position, .. } => start_position,
                    FileBlock::File { start_position, .. } => start_position
                }));
                true
            }
        }
    }

    fn checksum(&self) -> usize {
        self.files.iter().map(|f| match f {
            FileBlock::Empty { .. } => 0,
            FileBlock::File {start_position, id, size} => {
                let mut acc = 0;
                for i in *start_position..(start_position+size) {
                    acc += i*id;
                };
                acc
            }
        }).sum()
    }
}

#[part_two]
fn part_two(input: String) -> usize {
    let mut fs = Filesystem2::from(input);

    

    for file_id in (0..=fs.max_id).rev() {
        let file_to_move_pos = fs.position(file_id);
        for i in 0..file_to_move_pos {
            if fs.try_move(file_to_move_pos, i) {
                break;
            }
        }

    }

    fs.checksum()
}

harness!(part_1: 6390180901651, part_2: 6412390114238);
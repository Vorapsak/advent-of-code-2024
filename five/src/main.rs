use advent::prelude::*;

#[derive(Debug)]
struct Rule {
    first: u32,
    second: u32,
}

#[derive(Debug)]
struct Book {
    pages: Vec<u32>
}

impl Rule {
    fn from(line: &str) -> Rule {
        let (a, b) = line.split_once("|").unwrap();
        Rule {
            first: a.parse().unwrap(),
            second: b.parse().unwrap()
        }
    }
}

impl Book {
    fn from(line: &str) -> Book {
        let v: Vec<u32> = line.split(",").map(|num| num.parse().unwrap()).collect();
        Book {
            pages: v
        }
    }

    fn satisfies(self: &Self, rules: &Vec<Rule>) -> bool {
        rules.iter()
        .all(|rule| 
            match (self.pages.iter().position(|&x| x == rule.first), self.pages.iter().position(|&x| x == rule.second)) {
                (Some(a), Some(b)) => a < b,
                (None, _) => true,
                (_, None) => true
            }
        )
    }
}

#[part_one]
fn part_one(input: String) -> u32 {
    let (rules, books) = input.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules.lines().map(|line| Rule::from(line)).collect();
    let books: Vec<Book> = books.lines().map(|line| Book::from(line)).collect();
    
    books.iter()
        .filter(|&book| book.satisfies(&rules))
        .map(|book| book.pages[book.pages.len()/2])
        .sum()
}


fn fixup(book: &Book, rules: &Vec<Rule>) -> Book {
    let mut pages = book.pages.clone();
    rules.iter().for_each(|rule|
        match (pages.iter().position(|&x| x == rule.first), pages.iter().position(|&x| x == rule.second)) {
            (Some(a), Some(b)) => if a > b { pages.swap(a, b)},
            (None, _) => (),
            (_, None) => (),
        }
    );
    
    Book {
        pages: pages
    }
}


#[part_two]
fn part_two(input: String) -> u32 {

    let (rules, books) = input.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules.lines().map(|line| Rule::from(line)).collect();
    let books: Vec<Book> = books.lines().map(|line| Book::from(line)).collect();
    
    let mut fixed: Vec<Book> = books.iter()
        .filter(|&book| !book.satisfies(&rules))
        .map(|book| fixup(&book, &rules)).collect();

    while fixed.iter().filter(|&book| book.satisfies(&rules)).count() != fixed.len() {
            fixed = fixed.iter().map(|book| fixup(&book, &rules)).collect();
    }


    fixed.iter()
        .map(|book| book.pages[book.pages.len()/2])
        .sum()

}

harness!(part_1: 5588, part_2: 5331);
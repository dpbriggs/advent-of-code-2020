use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
    Open,
    Tree,
}

#[derive(Debug, Default)]
struct Hill {
    layout: Vec<Vec<Tile>>,
    curr_row: usize,
    curr_col: usize,
    col_size: usize,
}

impl Hill {
    fn from_input_file(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<Hill> {
        let mut layout = Vec::new();
        let mut col_size = 0;
        for line in lines {
            let mut curr_line = Vec::new();
            for c in line?.chars() {
                match c {
                    '.' => curr_line.push(Tile::Open),
                    '#' => curr_line.push(Tile::Tree),
                    _ => panic!("Unknown Char"),
                }
            }
            col_size = curr_line.len();
            layout.push(curr_line)
        }
        Ok(Hill {
            layout,
            col_size,
            curr_col: 0,
            curr_row: 0,
        })
    }

    fn at_index(&self, row: usize, col: usize) -> Option<Tile> {
        if row >= self.layout.len() {
            None
        } else {
            Some(self.layout[row][col % self.col_size])
        }
    }

    // Returns number of trees encountered
    fn move_toboggan(&mut self, down: usize, right: usize) -> Option<u32> {
        self.curr_row += down;
        self.curr_col += right;
        if self.at_index(self.curr_row, self.curr_col)? == Tile::Tree {
            Some(1)
        } else {
            Some(0)
        }
    }

    fn reset_position(&mut self) {
        self.curr_col = 0;
        self.curr_row = 0;
    }

    fn run_slope(&mut self, down: usize, right: usize) -> u64 {
        let mut trees_encountered = 0;
        while let Some(tree) = self.move_toboggan(down, right) {
            trees_encountered += tree as u64;
        }
        self.reset_position();
        trees_encountered
    }
}

#[allow(unused)]
fn problem_1(hill: &mut Hill) {
    let trees_encountered = hill.run_slope(1, 3);
    println!("Trees encountered: {}", trees_encountered);
}

#[allow(unused)]
fn problem_2(hill: &mut Hill) {
    let run_1 = hill.run_slope(1, 1);
    let run_2 = hill.run_slope(1, 3);
    let run_3 = hill.run_slope(1, 5);
    let run_4 = hill.run_slope(1, 7);
    let run_5 = hill.run_slope(2, 1);
    println!(
        "{} * {} * {} * {} * {} == {}",
        run_1,
        run_2,
        run_3,
        run_4,
        run_5,
        run_1 * run_2 * run_3 * run_4 * run_5
    )
}

fn open_input(file_name: &'static str) -> io::Result<Hill> {
    let f = File::open(file_name)?;
    Hill::from_input_file(BufReader::new(f).lines())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut hill = open_input("input.txt")?;
    // problem_1(&mut hill);
    problem_2(&mut hill);
    Ok(())
}

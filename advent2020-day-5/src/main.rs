use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn open_input(file_name: &'static str) -> io::Result<BufReader<File>> {
    let f = File::open(file_name)?;
    Ok(BufReader::new(f))
}

fn binary_search(low: u32, high: u32, left_char: char, right_char: char, inst: &str) -> u32 {
    let mut low = low;
    let mut high = high;
    for c in inst.chars() {
        let mid = (low + high) / 2;
        if c == left_char {
            high = mid;
        } else if c == right_char {
            low = mid;
        } else {
            panic!("Unknown char: {}", c)
        }
    }
    return high;
}

fn seat_id(line: &str) -> u32 {
    assert!(line.len() == 10);
    let row = binary_search(0, 127, 'F', 'B', &line[0..7]);
    let col = binary_search(0, 7, 'L', 'R', &line[7..]);
    row * 8 + col
}

fn problem_1(lines: BufReader<File>) {
    let max = lines
        .lines()
        .filter_map(|line| line.map(|s| seat_id(&s)).ok())
        .max()
        .unwrap();
    println!("max: {}", max);
}

fn problem_2(lines: BufReader<File>) {
    let mut seats: Vec<u32> = lines
        .lines()
        .filter_map(|line| line.map(|s| seat_id(&s)).ok())
        .collect();
    seats.sort();
    for w in seats.windows(2) {
        if w[0] + 1 != w[1] {
            println!("Your seat is: {}", w[0] + 1);
            return;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let seats = open_input("input.txt")?;
    // problem_1(seats);
    problem_2(seats);
    Ok(())
}

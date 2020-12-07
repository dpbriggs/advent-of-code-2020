use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Password {
    c: char,
    lower: u32,
    upper: u32,
    pass: String,
}

impl Password {
    fn from_input_line(line: &str, r: &Regex) -> Password {
        for cap in r.captures_iter(line) {
            return Password {
                c: cap[3].chars().next().unwrap(),
                lower: cap[1].parse().unwrap(),
                upper: cap[2].parse().unwrap(),
                pass: cap[4].into(),
            };
        }
        panic!("Cannot reach here right")
    }

    fn is_valid_frequency(&self) -> bool {
        let num_appeared = self
            .pass
            .chars()
            .fold(0, |acc, x| if x == self.c { acc + 1 } else { acc });
        self.lower <= num_appeared && num_appeared <= self.upper
    }

    fn is_valid_positionally(&self) -> bool {
        let first = self.pass.chars().nth((self.lower - 1) as usize).unwrap() == self.c;
        let second = self.pass.chars().nth((self.upper - 1) as usize).unwrap() == self.c;
        if first && second {
            false
        } else if first || second {
            true
        } else {
            false
        }
    }
}

fn open_input() -> io::Result<Vec<Password>> {
    let f = File::open("input.txt")?;
    let mut res = Vec::new();
    let r = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
    for line in BufReader::new(f).lines() {
        res.push(Password::from_input_line(&line?, &r));
    }
    Ok(res)
}

fn problem_1(input: &[Password]) {
    let num_valid = input
        .iter()
        .filter(|p| p.is_valid_frequency())
        .fold(0, |acc, x| acc + 1);
    println!("{} are valid", num_valid) // 393
}

fn problem_2(input: &[Password]) {
    let num_valid = input
        .iter()
        .filter(|p| p.is_valid_positionally())
        .fold(0, |acc, x| acc + 1);
    println!("{} are valid", num_valid) // 690
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = open_input()?;
    problem_1(&input);
    // problem_2(&input);
    Ok(())
}

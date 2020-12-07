#![feature(iterator_fold_self)]

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct ProperlyHandingNewLines<T: Iterator<Item = String>> {
    lines: T,
}

impl<T: Iterator<Item = String>> ProperlyHandingNewLines<T> {
    fn new(lines: T) -> Self {
        Self { lines }
    }
}

impl<T: Iterator<Item = String>> Iterator for ProperlyHandingNewLines<T> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = Vec::new();
        while let Some(line) = self.lines.next() {
            if line.is_empty() {
                return Some(res);
            }
            res.push(line);
        }
        None
    }
}

fn problem_1(input: impl Iterator<Item = Vec<String>>) {
    let sum: usize = input
        .into_iter()
        .map(|group| {
            let mut questions = HashSet::new();
            for answer in group {
                for c in answer.chars() {
                    questions.insert(c);
                }
            }
            questions.len()
        })
        .sum();
    println!("sum: {}", sum);
}

fn problem_2(input: impl Iterator<Item = Vec<String>>) {
    let sum: usize = input
        .into_iter()
        .map(|group| {
            group
                .iter()
                .map(|answers| answers.chars().collect::<HashSet<char>>())
                .fold_first(|acc, x| acc.intersection(&x).cloned().collect())
                .unwrap()
                .len()
        })
        .sum();
    println!("sum: {}", sum);
}

fn open_input(file_name: &'static str) -> io::Result<BufReader<File>> {
    let f = File::open(file_name)?;
    Ok(BufReader::new(f))
}

fn main() -> Result<(), Box<dyn Error>> {
    let forms = open_input("input.txt")?;
    // problem_1(ProperlyHandingNewLines::new(
    //     forms.lines().filter_map(Result::ok),
    // ));
    problem_2(ProperlyHandingNewLines::new(
        forms.lines().filter_map(Result::ok),
    ));
    Ok(())
}

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn open_input() -> io::Result<Vec<u32>> {
    let f = File::open("input.txt")?;
    let mut res = Vec::new();
    for line in BufReader::new(f).lines() {
        res.push(line?.parse().unwrap());
    }
    Ok(res)
}

fn problem_1(input: &[u32]) {
    let mut curr_index = 0;
    for i in input.iter().rev() {
        let sum = input[curr_index] + i;
        if sum == 2020 {
            println!("{} + {} == 2020", input[curr_index], i);
            println!("{} * {} == {}", input[curr_index], i, input[curr_index] * i);
        }
        if sum < 2020 {
            curr_index += 1;
            continue;
        }
    }
}

// It turns out that computers are fast
fn problem_2(input: &[u32]) {
    for i in input {
        for j in input {
            for k in input {
                let sum = i + j + k;
                if sum == 2020 {
                    println!("{} + {} + {} == 2020", i, j, k);
                    println!("{} * {} * {} == {}", i, j, k, i * j * k);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = open_input()?;
    input.sort();
    problem_2(&input);
    Ok(())
}

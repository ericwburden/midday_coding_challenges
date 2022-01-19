use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum LocalError {
    ParseIntError(std::num::ParseIntError),
    IOError(std::io::Error),
}

impl From<std::num::ParseIntError> for LocalError {
    fn from(e: std::num::ParseIntError) -> LocalError {
        LocalError::ParseIntError(e)
    }
}

impl From<std::io::Error> for LocalError {
    fn from(e: std::io::Error) -> LocalError {
        LocalError::IOError(e)
    }
}

fn read_input<P: AsRef<Path>>(p: P) -> Result<Vec<i32>, LocalError> {
    let file = File::open(p)?;
    io::BufReader::new(file)
        .lines()
        .flatten()
        .map(|s| s.parse::<i32>().map_err(LocalError::from))
        .collect::<Result<_,_>>()
}

fn part_one(input: &[i32]) -> i32 {
    let mut increases = 0;
    for idx in 1..input.len() {
        let (curr, prev) = (input[idx], input[idx-1]);
        if curr > prev { increases += 1; }
    }
    increases
}

fn part_two(input: &[i32]) -> i32 {
    let mut increases = 0;
    for idx in 3..input.len() {
        let (curr, prev) = (input[idx], input[idx-3]);
        if curr > prev { increases += 1; }
    }
    increases
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn it_works() {
        let input_path = Path::new("./input.txt");
        let numbers = read_input(input_path).unwrap();

        let start = Instant::now();
        let part_one_result = part_one(&numbers);
        let elapsed = start.elapsed();
        println!("Part One: {} in {:?}", part_one_result, elapsed);

        let start = Instant::now();
        let part_two_result = part_two(&numbers);
        let elapsed = start.elapsed();
        println!("Part Two: {} in {:?}", part_two_result, elapsed);
    }
}

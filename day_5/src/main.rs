use regex::Regex;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

fn part_1_and_2<R: Read>(io: R) -> () {
    let br = BufReader::new(io);
    let mut part_1_crates: Vec<Vec<char>> = vec![vec![]; 9];
    let mut part_2_crates: Vec<Vec<char>> = vec![vec![]; 9];
    let crates_re = Regex::new(r"^\D(\D)\D\s\D(\D)\D\s\D(\D)\D\s\D(\D)\D\s\D(\D)\D\s\D(\D)\D\s\D(\D)\D\s\D(\D)\D\s\D(\D)\D").unwrap();
    let instructions_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for line in br.lines() {
        let l: &str = &line.unwrap();
        if crates_re.is_match(l) {
            let captures = crates_re.captures(l).unwrap();
            for i in 1..10 {
                let c = captures[i].parse::<char>().unwrap();
                if c != " ".chars().next().unwrap() {
                    part_1_crates[i-1].insert(0, c);
                    part_2_crates[i-1].insert(0, c);
                }
            }
        }
        else if instructions_re.is_match(l) {
            let captures = instructions_re.captures(l).unwrap();
            let captures = [captures[1].parse::<usize>().unwrap(), captures[2].parse::<usize>().unwrap(), captures[3].parse::<usize>().unwrap()];
            let mut grabbed_crates = vec![];
            for _i in 0..captures[0] {
                let part_1_grabbed_crate = part_1_crates[captures[1]-1].pop().unwrap();
                let part_2_grabbed_crate = part_2_crates[captures[1]-1].pop().unwrap();

                part_1_crates[captures[2]-1].push(part_1_grabbed_crate);
                grabbed_crates.push(part_2_grabbed_crate);
            }
            grabbed_crates.reverse();
            part_2_crates[captures[2]-1].append(&mut grabbed_crates);
        }
    }
    println!("{:?}", part_1_crates);
    println!("{:?}", part_2_crates);
}


fn main() -> Result<(), Error> {

    part_1_and_2(File::open("input.txt")?);

    Ok(())
}

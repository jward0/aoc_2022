use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn part_1<R: Read>(io: R) -> Result<usize, Error> {

    let mut br = BufReader::new(io);
    let mut line = String::new();
    br.read_line(&mut line)?;

    let characters: Vec<char> = line.chars().collect();
    let windows = characters.windows(4);

    for (i, window) in windows.enumerate() {
        let mut uniq = HashSet::new();
        if window.into_iter().all(|x| uniq.insert(x)) {
            return Ok(i+4);
        }
    }

    return Ok(0);
}


fn part_2<R: Read>(io: R) -> Result<usize, Error> {

    let mut br = BufReader::new(io);
    let mut line = String::new();
    br.read_line(&mut line)?;

    let characters: Vec<char> = line.chars().collect();
    let windows = characters.windows(14);

    for (i, window) in windows.enumerate() {
        let mut uniq = HashSet::new();
        if window.into_iter().all(|x| uniq.insert(x)) {
            return Ok(i+14);
        }
    }

    return Ok(0);
}


fn main() -> Result<(), Error> {

    println!("{:?}", part_1(File::open("input.txt").unwrap()));
    println!("{:?}", part_2(File::open("input.txt").unwrap()));

    Ok(())
}

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn part_1<R: Read>(io: R) -> () {
    let br = BufReader::new(io);
    let mut running_total: i64 = 0;
    for line in br.lines() {
        let l: Vec<char> = line.unwrap().chars().collect();
        let opponent: i64;
        let mine: i64;

        match l[0] {
            'A' => opponent = 1,
            'B' => opponent = 2,
            'C' => opponent = 3,
            _   => unreachable!()
        };

        match l[2] {
            'X' => mine = 1,
            'Y' => mine = 2,
            'Z' => mine = 3,
            _   => unreachable!()
        };

        match mine - opponent {
            0  => running_total += 3,
            1  => running_total += 6,
            -2 => running_total += 6,
            _  => running_total += 0
        };

        running_total += mine;

    }
    println!("{}", running_total);
}


fn part_2<R: Read>(io: R) -> () {
    let br = BufReader::new(io);
    let mut running_total: i64 = 0;
    for line in br.lines() {
        let l: Vec<char> = line.unwrap().chars().collect();
        let opponent: i64;

        match l[0] {
            'A' => opponent = 1,
            'B' => opponent = 2,
            'C' => opponent = 3,
            _   => unreachable!()
        };

        match l[2] {
            'X' => running_total += 1 + (opponent + 1) % 3,
            'Y' => running_total += 3 + opponent,
            'Z' => running_total += 7 + opponent % 3,
            _   => unreachable!()
        };

    }
    println!("{}", running_total);
}



fn main() -> Result<(), Error> {

    part_1(File::open("input.txt")?);
    part_2(File::open("input.txt")?);

    Ok(())
}

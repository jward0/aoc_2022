use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn part_1<R: Read>(io: R) -> () {
    let br = BufReader::new(io);
    let mut running_total: i64 = 0;
    for line in br.lines() {
        let l: Vec<char> = line.unwrap().chars().collect();
        let first_half: Vec<char> = l[..l.len()/2].to_vec();
        let second_half: Vec<char> = l[l.len()/2..].to_vec();
        let mut flags: [bool; 127] = [false; 127];

        for item in first_half.iter() {
            flags[u32::from(*item) as usize] = true;
        }
        for item in second_half.iter() {
            if flags[u32::from(*item) as usize] {
                if u32::from(*item) >= 97 {
                    running_total += (u32::from(*item) as i64) - 96;
                    break;
                }
                else {
                    running_total += (u32::from(*item) as i64) - 38;
                    break;
                }
            }
        }
    }
    println!("{}", running_total);
}


fn part_2<R: Read>(io: R) -> () {
    let br = BufReader::new(io);
    let mut running_total: i64 = 0;
    for chunk in br.lines().map(|v| v.unwrap()).collect::<Vec<String>>().chunks(3) {
        let l1: Vec<char> = chunk[0].chars().collect();
        let l2: Vec<char> = chunk[1].chars().collect();
        let l3: Vec<char> = chunk[2].chars().collect();
        let mut flags: [[bool; 127]; 2] = [[false; 127]; 2];

        for item in l1.iter() {
            flags[0][u32::from(*item) as usize] = true;
        }
        for item in l2.iter() {
            flags[1][u32::from(*item) as usize] = true;
        }
        for item in l3.iter() {
            if flags[0][u32::from(*item) as usize] && flags[1][u32::from(*item) as usize] {
                if u32::from(*item) >= 97 {
                    running_total += (u32::from(*item) as i64) - 96;
                    break;
                }
                else {
                    running_total += (u32::from(*item) as i64) - 38;
                    break;
                }
            }
        }
    }
    println!("{}", running_total);
}


fn main() -> Result<(), Error> {

    part_2(File::open("input.txt")?);

    Ok(())
}

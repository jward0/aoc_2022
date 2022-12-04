use regex::Regex;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn both_parts<R: Read>(io: R) -> () {
    let br = BufReader::new(io);
    let mut part_1_running_total: i64 = 0;
    let mut part_2_running_total: i64 = 0;
    let re = Regex::new(r"^(\d+)\D+(\d+)\D+(\d+)\D+(\d+)$").unwrap();
    for line in br.lines() {
        let l: &str = &line.unwrap();
        let caps = re.captures(l).unwrap();
        let vec_caps: Vec<i32> = vec![caps[1].parse::<i32>().unwrap(), caps[2].parse::<i32>().unwrap(), caps[3].parse::<i32>().unwrap(), caps[4].parse::<i32>().unwrap()];
        if (vec_caps[0] >= vec_caps[2] && vec_caps[1] <= vec_caps[3]) || (vec_caps[0] <= vec_caps[2] && vec_caps[1] >= vec_caps[3]) {
            part_1_running_total += 1;
            part_2_running_total += 1
        }
        else if (vec_caps[0] >= vec_caps[2] && vec_caps[0] <= vec_caps[3]) || (vec_caps[1] >= vec_caps[2] && vec_caps[1] <= vec_caps[3]) {
            part_2_running_total += 1
        }

        
    }
    println!("{}", part_1_running_total);
    println!("{}", part_2_running_total);
}


fn main() -> Result<(), Error> {

    both_parts(File::open("input.txt")?);

    Ok(())
}

use regex::Regex;

use std::{
    cmp::{min, max},
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

fn part_1<R: Read>(io: R) -> Result<u64, Error> {

    let target_row: i64 = 2000000;
    let mut covered_regions: Vec<(i64, i64)> = vec![];
    let input_re = Regex::new(r"(-?\d+)").unwrap();

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = &line.unwrap();
        let captures = input_re.captures_iter(l);
        let mut captures_vec: Vec<i64> = vec![];
        for cap in captures {
            captures_vec.push(cap[1].to_string().parse::<i64>().unwrap());
        }
        // println!("{:?}", captures_vec);
        let distance_to_nearest = (captures_vec[0] - captures_vec[2]).abs() + (captures_vec[1] - captures_vec[3]).abs();
        let distance_to_target = (target_row - captures_vec[1]).abs();
        if distance_to_target < distance_to_nearest {
            let slack_distance = distance_to_nearest - distance_to_target;
            covered_regions.push((captures_vec[0] - slack_distance, captures_vec[0] + slack_distance));
        }
        // println!("{:?}", covered_regions);
    }

    let mut combined_covered_regions: Vec<(i64, i64)> = vec![];
    combined_covered_regions.push(covered_regions[0]);

    for i in 0..covered_regions.len() {
        for j in 0..combined_covered_regions.len() {
            let region = covered_regions[i];
            let c_region = combined_covered_regions[j];
            if region.0 <= c_region.1 || region.1 >= c_region.0 {
                combined_covered_regions[j] = (min(region.0, c_region.0), max(region.1, c_region.1));
            }
            else {
                combined_covered_regions.push(region);
            }
        }
    }

    let covered_spaces: u64 = combined_covered_regions.iter().map(|r| (r.1 - r.0) as u64).collect::<Vec<u64>>().iter().sum();

    Ok(covered_spaces)
}


fn part_2<R: Read>(io: R) -> Result<i64, Error> {

    
    let input_re = Regex::new(r"(-?\d+)").unwrap();
    let br = BufReader::new(io);
    let mut captures_vec: Vec<Vec<i64>> = vec![];

    for (idx, line) in br.lines().enumerate() {
        let l = &line.unwrap();
        captures_vec.push(vec![]);
        let captures = input_re.captures_iter(l);
        for cap in captures {
            captures_vec[idx].push(cap[1].to_string().parse::<i64>().unwrap());
        }
    }


    for y in 0..4000001 {

        let mut covered_regions: Vec<(i64, i64)> = vec![];

        for data in captures_vec.iter() {
            // println!("{:?}", captures_vec);
            let distance_to_nearest = (data[0] - data[2]).abs() + (data[1] - data[3]).abs();
            let distance_to_target = (y - data[1]).abs();
            if distance_to_target < distance_to_nearest {
                let slack_distance = distance_to_nearest - distance_to_target;
                covered_regions.push((max(data[0] - slack_distance, 0), min(data[0] + slack_distance, 4000000)));
            }
            // println!("{:?}", covered_regions);
        }

        let mut combined_covered_regions: Vec<(i64, i64)> = vec![];
        combined_covered_regions.push(covered_regions[0]);
        let mut add_flag: bool;
        for i in 0..covered_regions.len() {
            add_flag = true;
            for j in 0..combined_covered_regions.len() {
                let region = covered_regions[i];
                let c_region = combined_covered_regions[j];
                // println!("{:?}, {:?}", region, c_region);
                if region.0 <= c_region.1 && region.1 >= c_region.0 {
                    // println!("Combining...");
                    combined_covered_regions[j] = (min(region.0, c_region.0), max(region.1, c_region.1));
                    add_flag = false;
                }
                else if region.0 == c_region.1 + 1 {
                    // println!("Combining...");
                    combined_covered_regions[j] = (c_region.0, region.1);
                    add_flag = false;
                }
                else if region.1 == c_region.0 - 1 {
                    // println!("Combining...");
                    combined_covered_regions[j] = (region.0, c_region.1);
                    add_flag = false;
                }
            }
            if add_flag {
                // println!("Adding...");
                combined_covered_regions.push(covered_regions[i]);
            }
        }

        covered_regions = combined_covered_regions;
        combined_covered_regions = vec![];
        combined_covered_regions.push(covered_regions[0]);

        for i in 0..covered_regions.len() {
            add_flag = true;
            for j in 0..combined_covered_regions.len() {
                let region = covered_regions[i];
                let c_region = combined_covered_regions[j];
                // println!("{:?}, {:?}", region, c_region);
                if region.0 <= c_region.1 && region.1 >= c_region.0 {
                    // println!("Combining...");
                    combined_covered_regions[j] = (min(region.0, c_region.0), max(region.1, c_region.1));
                    add_flag = false;
                }
                else if region.0 == c_region.1 + 1 {
                    // println!("Combining...");
                    combined_covered_regions[j] = (c_region.0, region.1);
                    add_flag = false;
                }
                else if region.1 == c_region.0 - 1 {
                    // println!("Combining...");
                    combined_covered_regions[j] = (region.0, c_region.1);
                    add_flag = false;
                }
            }
            if add_flag {
                // println!("Adding...");
                combined_covered_regions.push(covered_regions[i]);
            }
        }

        let covered_spaces: u64 = combined_covered_regions.iter().map(|r| (r.1 - r.0 + 1) as u64).collect::<Vec<u64>>().iter().sum();
        if covered_spaces == 4000000 {
            let x: i64 = combined_covered_regions[0].1 + 1;
            return Ok(x * 4000000 + y);
        }
    }

    Ok(0)
}


fn main() {
    println!("{:?}", part_1(File::open("input.txt").unwrap()));
    println!("{:?}", part_2(File::open("input.txt").unwrap()));
}

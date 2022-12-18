use itertools::Itertools;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn part_1<R: Read>(io: R) -> Result<u32, Error> {

    let mut map: [[char; 140]; 200] = [['.'; 140]; 200];
    map[0][100] = '+';

    let br = BufReader::new(io);
    for line in br.lines() {
        let mut rock_path: Vec<(usize, usize)> = vec![];
        let l = &line.unwrap();
        let mut points = l.split(" -> ").collect::<Vec<&str>>();
        for (a, b) in points.iter().tuple_windows() {
            let start_coords = a.split(",").collect::<Vec<&str>>().iter().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let end_coords = b.split(",").collect::<Vec<&str>>().iter().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            if start_coords[0] < end_coords[0] || start_coords[1] < end_coords[1] {
                for x in start_coords[0]..end_coords[0]+1 {
                    for y in start_coords[1]..end_coords[1]+1 {
                        map[y][x-400] = '#';
                    }
                }
            } else {
                for x in end_coords[0]..start_coords[0]+1 {
                    for y in end_coords[1]..start_coords[1]+1 {
                        map[y][x-400] = '#';
                    }
                }
            }
        }
    }

    let mut sand_counter: u32 = 0;

    'outer: loop {
        let mut x: usize = 100;
        let mut y: usize = 0;
        loop {
            if y == map.len()-1 {
                break 'outer
            }
            if map[y+1][x] == '.' {
                y += 1;
                continue;
            } else if map[y+1][x-1] == '.' {
                y += 1;
                x -= 1;
                continue;
            } else if map[y+1][x+1] == '.' {
                y += 1;
                x += 1;
                continue;
            } else {
                map[y][x] = 'o';
                sand_counter += 1;
                break;
            }
        }
    }

    for line in map {
        let s: String = line.iter().collect();
        println!("{}", s);
    }

    Ok(sand_counter)
}

fn part_2<R: Read>(io: R) -> Result<u32, Error> {

    let mut map: [[char; 1000]; 169] = [['.'; 1000]; 169];
    map[168] = ['#'; 1000];
    map[0][500] = '+';

    let br = BufReader::new(io);
    for line in br.lines() {
        let mut rock_path: Vec<(usize, usize)> = vec![];
        let l = &line.unwrap();
        let mut points = l.split(" -> ").collect::<Vec<&str>>();
        for (a, b) in points.iter().tuple_windows() {
            let start_coords = a.split(",").collect::<Vec<&str>>().iter().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let end_coords = b.split(",").collect::<Vec<&str>>().iter().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            if start_coords[0] < end_coords[0] || start_coords[1] < end_coords[1] {
                for x in start_coords[0]..end_coords[0]+1 {
                    for y in start_coords[1]..end_coords[1]+1 {
                        map[y][x] = '#';
                    }
                }
            } else {
                for x in end_coords[0]..start_coords[0]+1 {
                    for y in end_coords[1]..start_coords[1]+1 {
                        map[y][x] = '#';
                    }
                }
            }
        }
    }

    let mut sand_counter: u32 = 0;

    'outer: loop {
        let mut x: usize = 500;
        let mut y: usize = 0;
        loop {
            if y == map.len()-1 {
                break 'outer
            }
            if map[y+1][x] == '.' {
                y += 1;
                continue;
            } else if map[y+1][x-1] == '.' {
                y += 1;
                x -= 1;
                continue;
            } else if map[y+1][x+1] == '.' {
                y += 1;
                x += 1;
                continue;
            } else {
                map[y][x] = 'o';
                sand_counter += 1;
                if (y, x) == (0, 500) {
                    break 'outer
                }
                break;
            }
        }
    }

    Ok(sand_counter)
}


fn main() {
    println!("{:?}", part_1(File::open("input.txt").unwrap()));
    println!("{:?}", part_2(File::open("input.txt").unwrap()));
}

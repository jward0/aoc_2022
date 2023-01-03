use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

fn part_1<R: Read>(io: R) -> Result<u32, Error> {

    let mut droplet: [[[u32; 30]; 30]; 30] = [[[0; 30]; 30]; 30];

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = &line.unwrap();
        let coords = l.split(',').map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        println!("{:?}", coords);
        droplet[coords[0]][coords[1]][coords[2]] = 6;
        //x neighbours
        if coords[0] != 29 {
            if droplet[coords[0]+1][coords[1]][coords[2]] != 0 {
                droplet[coords[0]+1][coords[1]][coords[2]] -= 1;
                droplet[coords[0]][coords[1]][coords[2]] -= 1;
            }
        }
        if coords[0] != 0 {
            if droplet[coords[0]-1][coords[1]][coords[2]] != 0 {
                droplet[coords[0]-1][coords[1]][coords[2]] -= 1;
                droplet[coords[0]][coords[1]][coords[2]] -= 1;
            }
        }
        //y neighbours
        if coords[1] != 29 {
            if droplet[coords[0]][coords[1]+1][coords[2]] != 0 {
                droplet[coords[0]][coords[1]+1][coords[2]] -= 1;
                droplet[coords[0]][coords[1]][coords[2]] -= 1;
            }
        }
        if coords[1] != 0 {
            if droplet[coords[0]][coords[1]-1][coords[2]] != 0 {
                droplet[coords[0]][coords[1]-1][coords[2]] -= 1;
                droplet[coords[0]][coords[1]][coords[2]] -= 1;
            }
        }
        //z neighbours
        if coords[2] != 29 {
            if droplet[coords[0]][coords[1]][coords[2]+1] != 0 {
                droplet[coords[0]][coords[1]][coords[2]+1] -= 1;
                droplet[coords[0]][coords[1]][coords[2]] -= 1;
            }
        }
        if coords[2] != 0 {
            if droplet[coords[0]][coords[1]][coords[2]-1] != 0 {
                droplet[coords[0]][coords[1]][coords[2]-1] -= 1;
                droplet[coords[0]][coords[1]][coords[2]] -= 1;
            }
        }
    }

    let surface_area = droplet.map(|arr_2d| arr_2d.map(
        |arr| arr.iter().sum()).iter().sum()).iter().sum();
    

    Ok(surface_area)
}

fn main() {
    println!("{:?}", part_1(File::open("input.txt").unwrap()));
}

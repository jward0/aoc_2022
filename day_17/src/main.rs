use std::{
    fs::{File, read_to_string},
    io::{BufRead, BufReader, Error, Read},
};

#[derive(Debug)]
struct Rock {
    height: usize,
    shape: Vec<[u8; 9]>,
}

impl Rock {

    fn move_left(&mut self) {
        for i in 0..self.shape.len() {
            for j in 0..8 {
                self.shape[i][j] = self.shape[i][j+1];
            }
            self.shape[i][8] = 0;
        }
    }

    fn move_right(&mut self) {
        for i in 0..self.shape.len() {
            for j in (1..9).rev() {
                self.shape[i][j] = self.shape[i][j-1];
            }
            self.shape[i][0] = 0;
        }

    }
    fn move_rock(&mut self, direction: char, space: &Vec<[u8; 9]>) -> bool {
        // attempt movement left or right
        //println!("{:?}", self.shape);
        match direction {
            '<' => {
                self.move_left();
                // println!("Moving left...");
                if self.check_collision(&space) {
                    // println!("Blocked!");
                    //println!("Collision");
                    self.move_right();
                }
            },
            '>' => {
                self.move_right();
                // println!("Moving right...");
                if self.check_collision(&space) {
                    // println!("Blocked!");
                    self.move_left();
                } 
            },
            _ => unreachable!(),
        }
        // attempt movement downwards, return false if the rock cannot move downwards, true if it can
        self.height -= 1;
        // println!("Moving down...");
        if self.check_collision(&space) {
            // println!("Stopped!");
            self.height += 1;
            return false
        }
        return true
    }

    fn new(shape_number: usize, height: usize) -> Rock {
        let mut shape: Vec<[u8; 9]> = vec![];
        match shape_number {
            0 => {
                shape = vec![[0, 0, 0, 1, 1, 1, 1, 0, 0]];
            },
            1 => {
                shape = vec![[0, 0, 0, 0, 1, 0, 0, 0, 0],
                             [0, 0, 0, 1, 1, 1, 0, 0, 0],
                             [0, 0, 0, 0, 1, 0, 0, 0, 0]];
            },
            2 => {
                shape = vec![[0, 0, 0, 1, 1, 1, 0, 0, 0],
                             [0, 0, 0, 0, 0, 1, 0, 0, 0],
                             [0, 0, 0, 0, 0, 1, 0, 0, 0]];
            },
            3 => {
                shape = vec![[0, 0, 0, 1, 0, 0, 0, 0, 0],
                             [0, 0, 0, 1, 0, 0, 0, 0, 0],
                             [0, 0, 0, 1, 0, 0, 0, 0, 0],
                             [0, 0, 0, 1, 0, 0, 0, 0, 0]];
            },
            4 => {
                shape = vec![[0, 0, 0, 1, 1, 0, 0, 0, 0],
                             [0, 0, 0, 1, 1, 0, 0, 0, 0]];
            },
            _ => unreachable!(),
        }
        return Rock{height: height, shape: shape};
    }

    fn check_collision(&self, space: &Vec<[u8; 9]>) -> bool {
        // println!("Checking for collision between {:?}, {:?}", self.shape, space);
        for i in self.height..(self.height + self.shape.len()) {
            if space.len() > i {
                for j in 0..9 {
                    if self.shape[i - self.height][j] == 1 && space[i][j] == 1 {
                        // println!("Collision");
                        return true;
                    }
                }
            }
        }
        // println!("No collision");
        return false;
    }

    fn add_to(self, space: &Vec<[u8; 9]>) -> Vec<[u8; 9]> {

        let mut new_space = space.to_vec();
        for i in self.height..(self.height + self.shape.len()) {
            if new_space.len() <= i {
                new_space.push([1, 0, 0, 0, 0, 0, 0, 0, 1]);
            }
            for j in 1..8 {
                if self.shape[i - self.height][j] == 1 {
                    new_space[i][j] = 1;
                }
            }
        }

        return new_space;
    }
}

fn part_1() -> Result<usize, Error> {
    let mut space: Vec<[u8; 9]> = vec![[1; 9]];
    let jets = read_to_string("input.txt").unwrap();

    let jet_vec: Vec<char> = jets.chars().collect();
    let mut jet_len: usize = jet_vec.len();
    let mut jet_idx: usize = 0;
    let mut rock_idx: usize = 0;
    let mut last_height: usize = 4;
    loop {
        let mut rock = Rock::new(rock_idx % 5, last_height);
        // let mut rock = Rock::new(0, space.len() + 3);
        for _ in 0..3 {
            space.push([1, 0, 0, 0, 0, 0, 0, 0, 1]);
        }
        // println!("{:?}", rock);
        if rock_idx == 2022 {
            break;
        }
        loop{
            jet_idx += 1;
            if !rock.move_rock(jet_vec[((jet_idx-1) % jet_len)], &space) {
                last_height = &rock.height + &rock.shape.len() + 3;
                space = rock.add_to(&space);
                rock_idx += 1;
                break;
            }
        }
    }

    space.remove(0);

    loop {
        let top = space.pop().unwrap();
        if top[1..8].contains(&1) {
            space.push(top);
            break;
        }
    }

    space.reverse();

    for line in &space {
        println!("{:?}", line.map(|v| match v {
            1 => '#',
            0 => ' ',
            _ => unreachable!(),
        })[1..8].iter().collect::<String>());
    }

    Ok(space.len())
}

fn main() {
    println!("{:?}", part_1());
}

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

#[derive(Default, Clone, Copy)]
struct Node {
    cost: u64,
    height: i64,
    seen: bool,
    visited: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}


fn part_1<R: Read>(io: R) -> Result<u64, Error> {

    let mut map: Vec<Vec<Node>> = vec![];
    let mut priority_queue: Vec<(usize, usize)> = vec![];
    let mut target: (usize, usize) = (0, 0);

    let br = BufReader::new(io);
    for (i, line) in br.lines().enumerate() {
        let l = line.unwrap();
        map.push(vec![Default::default(); l.chars().count()]);
        for (j, height) in l.chars().enumerate() {
            map[i][j].height = (height as i64) - 97;
            map[i][j].cost = u64::MAX;
            match height {
                'S' => {priority_queue.push((i, j));
                        map[i][j].height = 0;
                        map[i][j].cost = 0},
                'E' => {target = (i, j);
                        map[i][j].height=25},
                _   => (),
            }
            if j > 0 {

                if map[i][j].height - map[i][j-1].height <= 1 {
                    map[i][j-1].right = true;
                }
                if map[i][j-1].height - map[i][j].height <= 1 {
                    map[i][j].left = true;
                }
            }
            if i > 0 {
                if map[i][j].height - map[i-1][j].height <= 1 {
                    map[i-1][j].down = true;
                }
                if map[i-1][j].height - map[i][j].height <= 1 {
                    map[i][j].up = true;
                }
            }
        }
    }

    let mut counter: u64 = 0;
    loop {
        counter += 1;
        let p = priority_queue.pop().unwrap();
        let i = p.0;
        let j = p.1;
        let c = map[i][j].cost;
        map[i][j].visited = true;
        if (i, j) == target {
            return Ok(c);
        }

        let mut valid_adjacent_points: Vec<(usize, usize)> = vec![];
        if map[i][j].up {valid_adjacent_points.push((i-1, j))};
        if map[i][j].down {valid_adjacent_points.push((i+1, j))};
        if map[i][j].left {valid_adjacent_points.push((i, j-1))};
        if map[i][j].right {valid_adjacent_points.push((i, j+1))};

        // ij_n means i,j coordinates of neighbour
        for ij_n in valid_adjacent_points.iter() {
            if !map[ij_n.0][ij_n.1].visited {
                if c + 1 < map[ij_n.0][ij_n.1].cost {
                    map[ij_n.0][ij_n.1].cost = c + 1;
                }
                if !map[ij_n.0][ij_n.1].seen {
                    priority_queue.push(*ij_n);
                    map[ij_n.0][ij_n.1].seen = true;
                }
            }
        }
        priority_queue.sort_by(|a, b| map[b.0][b.1].cost.partial_cmp(&map[a.0][a.1].cost).unwrap());

        if priority_queue.len() == 0 {
            println!("{}", counter);
            return Ok(u64::MAX)
        }
    }
}

fn part_2<R: Read>(io: R) -> Result<u64, Error> {

    let mut map: Vec<Vec<Node>> = vec![];
    let mut priority_queue: Vec<(usize, usize)> = vec![];
    let mut target: (usize, usize) = (0, 0);

    let br = BufReader::new(io);
    for (i, line) in br.lines().enumerate() {
        let l = line.unwrap();
        map.push(vec![Default::default(); l.chars().count()]);
        for (j, height) in l.chars().enumerate() {
            map[i][j].height = (height as i64) - 97;
            map[i][j].cost = u64::MAX;
            match height {
                'S' | 'a' => {priority_queue.push((i, j));
                        map[i][j].height = 0;
                        map[i][j].cost = 0},
                'E' => {target = (i, j);
                        map[i][j].height=25},
                _   => (),
            }
            if j > 0 {

                if map[i][j].height - map[i][j-1].height <= 1 {
                    map[i][j-1].right = true;
                }
                if map[i][j-1].height - map[i][j].height <= 1 {
                    map[i][j].left = true;
                }
            }
            if i > 0 {
                if map[i][j].height - map[i-1][j].height <= 1 {
                    map[i-1][j].down = true;
                }
                if map[i-1][j].height - map[i][j].height <= 1 {
                    map[i][j].up = true;
                }
            }
        }
    }

    let mut counter: u64 = 0;
    loop {
        counter += 1;
        let p = priority_queue.pop().unwrap();
        let i = p.0;
        let j = p.1;
        let c = map[i][j].cost;
        map[i][j].visited = true;
        if (i, j) == target {
            return Ok(c);
        }

        let mut valid_adjacent_points: Vec<(usize, usize)> = vec![];
        if map[i][j].up {valid_adjacent_points.push((i-1, j))};
        if map[i][j].down {valid_adjacent_points.push((i+1, j))};
        if map[i][j].left {valid_adjacent_points.push((i, j-1))};
        if map[i][j].right {valid_adjacent_points.push((i, j+1))};

        // ij_n means i,j coordinates of neighbour
        for ij_n in valid_adjacent_points.iter() {
            if !map[ij_n.0][ij_n.1].visited {
                if c + 1 < map[ij_n.0][ij_n.1].cost {
                    map[ij_n.0][ij_n.1].cost = c + 1;
                }
                if !map[ij_n.0][ij_n.1].seen {
                    priority_queue.push(*ij_n);
                    map[ij_n.0][ij_n.1].seen = true;
                }
            }
        }
        priority_queue.sort_by(|a, b| map[b.0][b.1].cost.partial_cmp(&map[a.0][a.1].cost).unwrap());

        if priority_queue.len() == 0 {
            println!("{}", counter);
            return Ok(u64::MAX)
        }
    }
}


fn main() {
    println!("{:?}", part_1(File::open("input.txt").unwrap()));
    println!("{:?}", part_2(File::open("input.txt").unwrap()));
}

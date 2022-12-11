use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

#[derive(Default, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn step_towards(&mut self, target: &Point) -> Vec<(i32, i32)> {
        let mut visited: Vec<(i32, i32)> = vec![];
        loop {
            if (target.x - self.x).abs() <= 1 && (target.y - self.y).abs() <= 1 {
                break
            }
            else {
                if target.x - self.x != 0 {
                    self.x += (target.x - self.x)/(target.x - self.x).abs();
                }
                if target.y - self.y != 0 {
                    self.y += (target.y - self.y)/(target.y - self.y).abs();
                }    
                visited.push((self.x, self.y));            
            }
        }
        visited
    }
}


fn part_1<R: Read>(io: R) -> Result<usize, Error> {

    let mut head: Point = Point{x: 0, y: 0};
    let mut tail: Point = Point{x: 0, y: 0};
    let mut unique_points = HashSet::new();
    unique_points.insert((tail.x, tail.y)); 

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = line.unwrap();
        let direction: char = l.chars().nth(0).unwrap();
        let distance: i32 = l[2..].parse::<i32>().unwrap();
        match direction {
            'U' => head.y += distance,
            'D' => head.y -= distance,
            'L' => head.x -= distance,
            'R' => head.x += distance,
            _   => unreachable!()
        }

        for point in tail.step_towards(&head).iter() {
            unique_points.insert(*point);
        }
    }

    Ok(unique_points.len())
}


fn part_2<R: Read>(io: R) -> Result<usize, Error> {

    let mut rope: [Point; 10] = Default::default();
    let mut unique_points = HashSet::new();
    unique_points.insert((rope[9].x, rope[9].y)); 

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = line.unwrap();
        let direction: char = l.chars().nth(0).unwrap();
        let distance: i32 = l[2..].parse::<i32>().unwrap();

        for _ in 1..distance + 1 {

            match direction {
                'U' => rope[0].y += 1,
                'D' => rope[0].y -= 1,
                'L' => rope[0].x -= 1,
                'R' => rope[0].x += 1,
                _   => unreachable!()
            }

            for i in 1..9 {
                rope[i].step_towards(&rope[i-1].clone());
            }
            for point in rope[9].step_towards(&rope[8].clone()).iter() {
                unique_points.insert(*point);
            }
        }
    }

    Ok(unique_points.len())
}



fn main() -> Result<(), Error> {

    println!("{:?}", part_1(File::open("input.txt").unwrap()));
    println!("{:?}", part_2(File::open("input.txt").unwrap()));
    Ok(())
}

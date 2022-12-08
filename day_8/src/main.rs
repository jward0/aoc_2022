use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};


fn part_1<R: Read>(io: R) -> Result<u32, Error> {

    let mut heights: Vec<Vec<u32>> = vec![];
    let mut visibility_flags: Vec<Vec<[u32; 4]>> = vec![];

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = line.unwrap();
        visibility_flags.push(vec![[0; 4]; l.chars().count()]);
        heights.push(l.chars().map(|v| v.to_digit(10).unwrap()).collect());
    }

    for (i, line) in heights.iter().enumerate() {
        for (j, height) in line.iter().enumerate() {
            // View from left
            if j == 0 || height > line[..j].iter().max().unwrap() {
                visibility_flags[i][j][0] = 1;
            }
            // View from top
            if i == 0 || height > heights[..i].iter().map(|s| s[j]).collect::<Vec<u32>>().iter().max().unwrap() {
                visibility_flags[i][j][1] = 1;
            }
            // View from right
            if j == line.len() - 1 || height > line[j+1..].iter().max().unwrap() {
                visibility_flags[i][j][2] = 1;
            }
            // View from bottom
            if i == heights.len() - 1 || height > heights[i+1..].iter().map(|s| s[j]).collect::<Vec<u32>>().iter().max().unwrap() {
                visibility_flags[i][j][3] = 1;
            }
        }
    }

    let tree_count: u32 = visibility_flags.iter().map(|line| line.iter().map(|item| item.iter().max().unwrap()).sum::<u32>()).sum::<u32>();

    Ok(tree_count)
}


fn part_2<R: Read>(io: R) -> Result<u32, Error> {

    let mut heights: Vec<Vec<u32>> = vec![];
    let mut visible_trees: Vec<Vec<[u32; 4]>> = vec![];

    let br = BufReader::new(io);
    for line in br.lines() {
        let l = line.unwrap();
        visible_trees.push(vec![[0; 4]; l.chars().count()]);
        heights.push(l.chars().map(|v| v.to_digit(10).unwrap()).collect());
    }

    for (i, line) in heights.iter().enumerate() {
        for (j, _height) in line.iter().enumerate() {

            // Look left
            for left_idx in (0..j).rev() {
                visible_trees[i][j][0] += 1;
                if line[left_idx] >= heights[i][j] {
                    break
                }
            }
            // Look right
            for right_idx in j+1..line.len() {
                visible_trees[i][j][1] += 1;
                if line[right_idx] >= heights[i][j] {
                    break
                }
            }
            // Look up
            for up_idx in (0..i).rev() {
                visible_trees[i][j][2] += 1;
                if heights[up_idx][j] >= heights[i][j] {
                    break
                }                
            }
            // Look down
            for down_idx in i+1..heights.len() {
                visible_trees[i][j][3] += 1;
                if heights[down_idx][j] >= heights[i][j] {
                    break
                }                 
            }
        }
    }

    let tree_count: Vec<Vec<u32>> = visible_trees.iter().map(
        |line| line.iter().map(
            |item| item[0]*item[1]*item[2]*item[3]
        ).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    let max_tree_count: u32 = *tree_count.iter().map(|v| v.iter().max().unwrap()).max().unwrap();

    Ok(max_tree_count)
}


fn main() -> Result<(), Error> {

    println!("{:?}", part_1(File::open("input.txt").unwrap()));
    println!("{:?}", part_2(File::open("input.txt").unwrap()));

    Ok(())
}

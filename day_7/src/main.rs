use regex::Regex;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

struct ArenaTree {
    arena: Vec<Node>,
}

struct Node {
    idx: usize,
    size: usize,
    is_file: bool,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(idx: usize, size: usize) -> Self {
        Self {
            idx,
            size, 
            is_file: false,
            parent: None,
            children: vec![],
        }
    }
}

impl ArenaTree {

    fn new() -> Self {
        Self {
            arena: vec![],
        }
    }

    fn node(&mut self, idx: usize, size: usize) -> usize {
        for node in &self.arena {
            if node.idx == idx {
                return node.idx;
            }
        }
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, size));
        idx
    }

    fn pass_up_the_chain(&mut self, idx: usize, value: usize) {
        if idx != 0 {
            let parent_idx = self.arena[idx].parent.unwrap();
            self.arena[parent_idx].size += value;
            self.pass_up_the_chain(parent_idx, value);
        }
    }

    fn propagate_sizes(&mut self) {
        for node in self.arena.iter_mut() {
            if node.size > 0 && node.is_file == false {
                node.is_file = true;
            }
        }
        for i in 1..self.arena.len() {
            if self.arena[i].is_file {
                self.pass_up_the_chain(i, self.arena[i].size);
            }
        }
    }
}


fn both_parts<R: Read>(io: R) -> Result<usize, Error> {

    let mut file_tree = ArenaTree::new();
    let mut current_fpath: Vec<usize> = vec![0];
    file_tree.node(0, 0);

    let re_cd = Regex::new(r"^\$ cd (.+)$").unwrap();
    let re_ls = Regex::new(r"^\$ ls$").unwrap();
    let re_dir = Regex::new(r"^dir (.+)$").unwrap();
    let re_file = Regex::new(r"^(\d+) (.+)$").unwrap();

    // Build file tree from input text
    let br = BufReader::new(io);

    for line in br.lines() {
        let l: &str = &line.unwrap();
        // Change directory
        if re_cd.is_match(l) {
            let destination = &re_cd.captures(l).unwrap()[1];
            if destination == ".." {
                current_fpath.pop();
            }
            else if destination == "/" {
                current_fpath = vec![0];
            }
            else {
                let ndx = file_tree.arena.len();
                file_tree.node(ndx, 0);
                file_tree.arena[ndx].parent = Some(current_fpath[current_fpath.len()-1]);
                file_tree.arena[current_fpath[current_fpath.len()-1]].children.push(ndx);
                current_fpath.push(ndx);
            }
        }
        // List all (do nothing)
        else if re_ls.is_match(l) {
            ();
        }
        // Found directory (do nothing)
        else if re_dir.is_match(l) {
            ();
        }
        // Found file (add file to tree)
        else if re_file.is_match(l) {
            let size = &re_file.captures(l).unwrap()[1].parse::<usize>().unwrap();
            let ndx = file_tree.arena.len();
            file_tree.node(ndx, *size);
            file_tree.arena[ndx].parent = Some(current_fpath[current_fpath.len()-1]);
            file_tree.arena[current_fpath[current_fpath.len()-1]].children.push(ndx);  
        }
    }

    file_tree.propagate_sizes();
    let mut total: usize = 0;

    for node in file_tree.arena.iter() {
        if node.size <= 100000 && !node.is_file {
            total += node.size;
        }
    }

    println!("Part 1 solution:");
    println!("{}", total);

    let available_space: usize = 70000000 - file_tree.arena[0].size;
    let mut current_best_val: usize = available_space + file_tree.arena[0].size;
    let mut current_best_ndx: usize = 0;

    for node in file_tree.arena.iter() {
        if available_space + node.size < current_best_val && available_space + node.size >= 30000000 {
            current_best_val = available_space + node.size;
            current_best_ndx = node.idx;
        }
    }

    println!("Part 2 solution:");
    println!("{}", file_tree.arena[current_best_ndx].size);

    return Ok(0);
}


fn main() -> Result<(), Error> {

    println!("{:?}", both_parts(File::open("input.txt").unwrap()));

    Ok(())
}

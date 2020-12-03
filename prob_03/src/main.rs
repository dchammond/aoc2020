use std::fs;
use std::env;
use std::path::*;

struct TreeLine {
    line: Vec<u8>
}

impl TreeLine {
    pub fn new(s: &str) -> Self {
        TreeLine { line: s.as_bytes().to_vec() }
    }
    pub fn get(&self, idx: usize) -> u8 {
        self.line[idx % self.line.len()]
    }
    pub fn is_tree(c: u8) -> bool {
        c == b'#'
    }
}

fn count_encounters(down: usize, right: usize, forest: &[TreeLine]) -> usize {
    let mut pos = (0usize,0usize);
    let finish_line = forest.len();
    let mut tree_count = 0usize;
    while pos.0 < finish_line {
        if TreeLine::is_tree(forest[pos.0].get(pos.1)) {
            tree_count += 1;
        }
        pos = (pos.0 + down, pos.1 + right);
    }
    tree_count
}

fn main() {
    let file_path = PathBuf::from(env::args().last().unwrap());
    let data = fs::read_to_string(file_path).unwrap();
    let forest: Vec<TreeLine> = data.lines().into_iter().map(|l| {
        TreeLine::new(l)
    }).collect();
    let mut result = 1;
    for x in [(1,1),(1,3),(1,5),(1,7),(2,1)].iter() {
        result *= count_encounters(x.0, x.1, &forest);
    }
    println!("{}", result);
}

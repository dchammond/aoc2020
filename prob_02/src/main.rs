use std::fs;
use std::env;
use std::path::*;

fn is_valid(min: u8, max: u8, needle: char, passwd: &str) -> bool {
    let mut count = 0;
    for c in passwd.chars() {
        if needle == c {
            count += 1;
        }
    }
    (min <= count) && (count <= max)
}

fn is_valid2(idx1: u8, idx2: u8, needle: char, passwd: &str) -> bool {
    (passwd.as_bytes()[(idx1-1) as usize] as char == needle) ^ (passwd.as_bytes()[(idx2-1) as usize] as char == needle)
}

fn parse_line(line: &str) -> (u8, u8, char, &str) {
    let split: Vec<&str> = line.split(' ').collect();
    let range_split: Vec<&str> = split[0].split('-').collect();
    let min = range_split[0].parse().unwrap();
    let max = range_split[1].parse().unwrap();
    let c = split[1].chars().next().unwrap();
    let s = split[2];
    (min, max, c, s)
}

fn main() {
    let file_path = PathBuf::from(env::args().last().unwrap());
    let data = fs::read_to_string(file_path).unwrap();
    let num = data.lines().into_iter().filter(|l| {
        let parsed = parse_line(l);
        is_valid2(parsed.0, parsed.1, parsed.2, parsed.3)
    }).count();
    println!("{}", num);
}

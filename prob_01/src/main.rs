use std::env;
use std::path::*;
use std::ffi::OsStr;
use std::fs;

fn main() {
    let file_path = PathBuf::from(env::args().last().unwrap());
    let data = fs::read_to_string(file_path).unwrap();
    let mut lines = data.lines();
    let mut nums: Vec<u32> = lines.map(|l| l.parse::<u32>().unwrap()).collect();
    nums.sort_unstable();
    nums = nums.into_iter().filter(|&x| x <= 2020).collect();
    nums.reverse();
    'outer: for i in (1..nums.len()).rev() {
        let x = nums[i];
        for num in &nums[0..i] {
            if num + x == 2020 {
                println!("{} + {} == 2020, {} * {} == {}", num, x, num, x, num*x);
                break 'outer;
            }
        }
    }
    for i in (1..nums.len()).rev() {
        let x = nums[i];
        let y = nums[i - 1];
        for num in &nums[0..i-1] {
            if num + x + y == 2020 {
                println!("{} + {} + {} == 2020, {} * {} * {} == {}", num, x, y, num, x, y, num*x*y);
                return;
            }
        }
    }
}

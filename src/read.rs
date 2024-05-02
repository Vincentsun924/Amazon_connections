use std::fs::File;
use std::io::{BufRead, BufReader};



//reads the txt file and extract the values based on separeated tabs
pub fn read_file(path: &str) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        if let Ok(line_str) = line {
            let v: Vec<&str> = line_str.trim().split('\t').collect();
            if v.len() >= 2 {
                if let (Ok(x), Ok(y)) = (v[0].parse::<u32>(), v[1].parse::<u32>()) {
                    result.push((x, y));
                }
            }
        }
    }
    result
}
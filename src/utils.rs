use std::fs::File;
use std::io::Read;

pub const ROOT_PATH: &str = "./data";

pub fn read_file(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    return content;
}

pub fn read_file_as_strs(path: String) -> Vec<String> {
    let mut lines = Vec::new();
    let content = read_file(path);
    for line in content.split("\n") {
        if line.len() != 0 {
            lines.push(line.to_string())
        }
    }
    return lines;
}

pub fn read_file_as_chars(path: String) -> Vec<Vec<char>> {
    let mut lines = Vec::new();
    let content = read_file(path);
    for line in content.split("\n") {
        if line.len() != 0 {
            lines.push(line.chars().collect())
        }
    }
    return lines;
}

pub fn read_file_as_i32s(path: String) -> Vec<i32> {
    let mut lines = Vec::new();
    let content = read_file(path);
    for line in content.split("\n") {
        if line.len() != 0 {
            lines.push(line.parse::<i32>().unwrap())
        }
    }
    return lines;
}

pub fn base_path(day_no: &str) -> String {
    return format!("{}/{}", ROOT_PATH, day_no);
}

pub fn example_path(day_no: &str) -> String {
    return base_path(day_no) + "/example";
}

pub fn input_path(day_no: &str) -> String {
    return base_path(day_no) + "/input";
}

pub fn str_to_i32(s: String, radix: u32) -> i32 {
    return i32::from_str_radix(s.as_str(), radix).unwrap();
}

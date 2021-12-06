#![allow(dead_code)]
#[path = "./utils.rs"]
mod utils;

const DAY_NO: &str = "02";

fn solve_p1(path: String) -> i32 {
    let mut x: i32 = 0;
    let mut z: i32 = 0;
    let cmds = utils::read_file_as_strs(path);

    for cmd in cmds {
        let cols: Vec<&str> = cmd.split(" ").collect();
        let op = cols.get(0).unwrap();
        let val: i32 = cols.get(1).unwrap().to_string().parse().unwrap();
        if *op == "forward" {
            x += val;
        } else if *op == "down" {
            z += val;
        } else if *op == "up" {
            z -= val;
        }
    }
    return x * z;
}

fn solve_p2(path: String) -> i32 {
    let mut x: i32 = 0;
    let mut z: i32 = 0;
    let mut aim: i32 = 0;
    let cmds = utils::read_file_as_strs(path);

    for cmd in cmds {
        let cols: Vec<&str> = cmd.split(" ").collect();
        let op = cols.get(0).unwrap();
        let val: i32 = cols.get(1).unwrap().to_string().parse().unwrap();
        if *op == "forward" {
            x += val;
            z += val * aim;
        } else if *op == "down" {
            aim += val;
        } else if *op == "up" {
            aim -= val;
        }
    }
    return x * z;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let tests = [
            (utils::example_path(DAY_NO), 150),
            (utils::input_path(DAY_NO), 1936494),
        ];

        for test in tests {
            assert_eq!(solve_p1(test.0), test.1);
        }
    }

    #[test]
    fn test_p2() {
        let tests = [
            (utils::example_path(DAY_NO), 900),
            (utils::input_path(DAY_NO), 1997106066),
        ];

        for test in tests {
            assert_eq!(solve_p2(test.0), test.1);
        }
    }
}

#![allow(dead_code)]
#[path = "./utils.rs"]
mod utils;

const DAY_NO: &str = "01";

fn solve_p1(path: String) -> i32 {
    let mut cnt = 0;
    let nums = utils::read_file_as_i32s(path);
    for i in 0..(nums.len() - 1) {
        if nums.get(i) < nums.get(i + 1) {
            cnt += 1;
        }
    }
    return cnt;
}

fn solve_p2(path: String) -> i32 {
    let mut cnt = 0;
    let nums = utils::read_file_as_i32s(path);

    let mut i = 0 as usize;
    while i + 3 <= nums.len() - 1 {
        if nums.get(i) < nums.get(i + 3) {
            cnt += 1;
        }
        i += 1;
    }
    return cnt;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let tests = [
            (utils::example_path(DAY_NO), 7),
            (utils::input_path(DAY_NO), 1475),
        ];

        for test in tests {
            assert_eq!(solve_p1(test.0), test.1);
        }
    }

    #[test]
    fn test_p2() {
        let tests = [
            (utils::example_path(DAY_NO), 5),
            (utils::input_path(DAY_NO), 1516),
        ];

        for test in tests {
            assert_eq!(solve_p2(test.0), test.1);
        }
    }
}

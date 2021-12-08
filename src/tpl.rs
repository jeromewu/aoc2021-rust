#![allow(dead_code)]
#[path = "./utils.rs"]
mod utils;

const DAY_NO: &str = "";

fn solve_p1(path: String) -> i32 {
    let _ = path;
    return 0;
}

fn solve_p2(path: String) -> i32 {
    let _ = path;
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let tests = [
            (utils::example_path(DAY_NO), 0),
            (utils::input_path(DAY_NO), 0),
        ];

        for test in tests {
            assert_eq!(solve_p1(test.0), test.1);
        }
    }

    #[test]
    fn test_p2() {
        let tests = [
            (utils::example_path(DAY_NO), 0),
            (utils::input_path(DAY_NO), 0),
        ];

        for test in tests {
            assert_eq!(solve_p2(test.0), test.1);
        }
    }
}

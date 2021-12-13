#![allow(dead_code)]
#[path = "./utils.rs"]
mod utils;

use std::cmp;

const DAY_NO: &str = "05";

struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn new_line(line: String) -> Line {
    let nums = line
        .replace(" -> ", ",")
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    return Line {
        x1: nums[0],
        y1: nums[1],
        x2: nums[2],
        y2: nums[3],
    };
}

fn print_map(map: &Vec<Vec<i32>>, len: usize) {
    for i in 0..len {
        for j in 0..len {
            print!("{}", map[i][j]);
        }
        println!();
    }
}

fn count_points(map: &Vec<Vec<i32>>) -> i32 {
    let mut cnt = 0;
    for row in map {
        for c in row {
            if *c > 1 {
                cnt += 1;
            }
        }
    }
    return cnt;
}

fn solve_p1(path: String) -> i32 {
    const LEN: usize = 1001;
    let mut map = vec![vec![0; LEN]; LEN];
    let mut lines = vec![];
    for line in utils::read_file_as_strs(path) {
        lines.push(new_line(line));
    }

    for line in lines {
        if line.x1 == line.x2 || line.y1 == line.y2 {
            for i in cmp::min(line.x1, line.x2)..(cmp::max(line.x1, line.x2) + 1) {
                for j in cmp::min(line.y1, line.y2)..(cmp::max(line.y1, line.y2) + 1) {
                    map[j][i] += 1;
                }
            }
        }
    }

    return count_points(&map);
}

fn solve_p2(path: String) -> i32 {
    const LEN: usize = 1001;
    let mut map = vec![vec![0; LEN]; LEN];
    let mut lines = vec![];
    for line in utils::read_file_as_strs(path) {
        lines.push(new_line(line));
    }

    for line in lines {
        if line.x1 == line.x2 {
            for i in cmp::min(line.y1, line.y2)..(cmp::max(line.y1, line.y2) + 1) {
                map[i][line.x1] += 1;
            }
        } else if line.y1 == line.y2 {
            for i in cmp::min(line.x1, line.x2)..(cmp::max(line.x1, line.x2) + 1) {
                map[line.y1][i] += 1;
            }
        } else {
            for i in cmp::min(line.x1, line.x2)..(cmp::max(line.x1, line.x2) + 1) {
                for j in cmp::min(line.y1, line.y2)..(cmp::max(line.y1, line.y2) + 1) {
                    let x_diff = (i as i32 - line.x1 as i32).abs();
                    let y_diff = (j as i32 - line.y1 as i32).abs();

                    if x_diff == y_diff {
                        map[j][i] += 1;
                    }
                }
            }
        }
    }

    return count_points(&map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let tests = [
            (utils::example_path(DAY_NO), 5),
            (utils::input_path(DAY_NO), 5774),
        ];

        for test in tests {
            assert_eq!(solve_p1(test.0), test.1);
        }
    }

    #[test]
    fn test_p2() {
        let tests = [
            (utils::example_path(DAY_NO), 12),
            (utils::input_path(DAY_NO), 18423),
        ];

        for test in tests {
            assert_eq!(solve_p2(test.0), test.1);
        }
    }
}

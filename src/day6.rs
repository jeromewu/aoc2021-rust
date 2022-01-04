#![allow(dead_code)]
#[path = "./utils.rs"]
mod utils;

const DAY_NO: &str = "06";

fn get_stats(path: String) -> Vec<i64> {
    let nums_str = utils::read_file(path);
    let mut stats = vec![0i64; 9];
    for n in nums_str.trim().split(',') {
        stats[n.parse::<usize>().unwrap()] += 1;
    }
    return stats;
}

fn cal_fishes(path: String, num_days: i32) -> i64 {
    let mut stats = get_stats(path);
    for _ in 0..num_days {
        let ready = stats[0];
        for j in 1..stats.len() {
            stats[j - 1] = stats[j];
        }
        stats[6] += ready;
        stats[8] = ready;
    }

    let mut sum = 0;
    for n in stats.iter() {
        sum += *n;
    }

    return sum;
}

fn solve_p1(path: String) -> i64 {
    return cal_fishes(path, 80);
}

fn solve_p2(path: String) -> i64 {
    return cal_fishes(path, 256);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let tests = [
            (utils::example_path(DAY_NO), 5934),
            (utils::input_path(DAY_NO), 362346),
        ];

        for test in tests {
            assert_eq!(solve_p1(test.0), test.1);
        }
    }

    #[test]
    fn test_p2() {
        let tests = [
            (utils::example_path(DAY_NO), 26984457539),
            (utils::input_path(DAY_NO), 1639643057051),
        ];

        for test in tests {
            assert_eq!(solve_p2(test.0), test.1);
        }
    }
}

#![allow(dead_code)]
#[path = "./utils.rs"]
mod utils;

const DAY_NO: &str = "03";

fn count_bits(nums: &Vec<Vec<char>>, nth: usize, is_most_mode: bool) -> char {
    let size = nums.len();
    let mut one_cnt = 0;
    for num in nums {
        one_cnt += if *num.get(nth).unwrap() == '1' { 1 } else { 0 };
    }
    if one_cnt * 2 > size {
        return if is_most_mode { '1' } else { '0' };
    } else if one_cnt * 2 < size {
        return if is_most_mode { '0' } else { '1' };
    }
    return if is_most_mode { '1' } else { '0' };
}

fn solve_p1(path: String) -> i32 {
    let lines = utils::read_file_as_chars(path);
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for nth in 0..lines.get(0).unwrap().len() {
        let is_most_one = count_bits(&lines, nth, true) == '1';
        gamma_rate += if is_most_one { "1" } else { "0" };
        epsilon_rate += if is_most_one { "0" } else { "1" };
    }
    return utils::str_to_i32(gamma_rate, 2) * utils::str_to_i32(epsilon_rate, 2);
}

fn find_rate(lines: &Vec<Vec<char>>, is_most_mode: bool) -> String {
    let mut nth = 0;
    let mut nums = lines.to_vec();
    while nums.len() != 1 {
        let bit = count_bits(&nums, nth, is_most_mode);
        nums = nums
            .into_iter()
            .filter(|num| *num.get(nth).unwrap() == bit)
            .collect();
        nth += 1;
    }
    return nums.get(0).unwrap().into_iter().collect();
}

fn solve_p2(path: String) -> i32 {
    let lines = utils::read_file_as_chars(path);
    let oxygen_rate = find_rate(&lines, true);
    let co2_rate = find_rate(&lines, false);
    return utils::str_to_i32(oxygen_rate, 2) * utils::str_to_i32(co2_rate, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let tests = [
            (utils::example_path(DAY_NO), 198),
            (utils::input_path(DAY_NO), 3923414),
        ];

        for test in tests {
            assert_eq!(solve_p1(test.0), test.1);
        }
    }

    #[test]
    fn test_p2() {
        let tests = [
            (utils::example_path(DAY_NO), 230),
            (utils::input_path(DAY_NO), 5852595),
        ];

        for test in tests {
            assert_eq!(solve_p2(test.0), test.1);
        }
    }
}

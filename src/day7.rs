#![allow(dead_code)]
#[path = "./utils.rs"]
mod utils;

const DAY_NO: &str = "07";

fn get_positions(path: String) -> Vec<i32> {
    let psts_str = utils::read_file(path);
    let mut psts = vec![];
    for p in psts_str.trim().split(',') {
        psts.push(p.parse::<i32>().unwrap());
    }
    return psts;
}

fn get_total_fuel(psts: &Vec<i32>, pos: i32) -> i32 {
    let mut sum = 0;
    for p in psts.iter() {
        sum += (*p - pos).abs();
    }
    return sum;
}

fn get_total_exp_fuel(psts: &Vec<i32>, pos: i32) -> i32 {
    let mut sum = 0;
    for p in psts.iter() {
        let diff = (*p - pos).abs();
        let fuel = (diff + 1) * diff / 2;
        sum += fuel
    }
    return sum;
}

fn get_min_max(psts: &Vec<i32>) -> (i32, i32) {
    let mut min = -1;
    let mut max = -1;

    for p in psts.iter() {
        if min == -1 || *p < min {
            min = *p;
        }
        if max == -1 || *p > max {
            max = *p;
        }
    }
    return (min, max);
}

fn solve_p1(path: String) -> i32 {
    let psts = get_positions(path);
    let (min, max) = get_min_max(&psts);

    let mut min_fuel = -1;
    for i in min..(max + 1) {
        let total_fuel = get_total_fuel(&psts, i);
        if min_fuel == -1 || total_fuel < min_fuel {
            min_fuel = total_fuel;
        }
    }
    return min_fuel;
}

fn solve_p2(path: String) -> i32 {
    let psts = get_positions(path);
    let (min, max) = get_min_max(&psts);

    let mut min_fuel = -1;
    for i in min..(max + 1) {
        let total_fuel = get_total_exp_fuel(&psts, i);
        if min_fuel == -1 || total_fuel < min_fuel {
            min_fuel = total_fuel;
        }
    }
    return min_fuel;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let tests = [
            (utils::example_path(DAY_NO), 37),
            (utils::input_path(DAY_NO), 345197),
        ];

        for test in tests {
            assert_eq!(solve_p1(test.0), test.1);
        }
    }

    #[test]
    fn test_p2() {
        let tests = [
            (utils::example_path(DAY_NO), 168),
            (utils::input_path(DAY_NO), 96361606),
        ];

        for test in tests {
            assert_eq!(solve_p2(test.0), test.1);
        }
    }
}

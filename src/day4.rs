#![allow(dead_code)]
#[path = "./utils.rs"]
mod utils;

const DAY_NO: &str = "04";

struct Board {
    size: usize,
    data: Vec<Vec<i32>>,
    win: bool,
}

impl Board {
    fn mark(&mut self, num: i32) {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.data[i][j] == num {
                    self.data[i][j] = -1;
                    return;
                }
            }
        }
    }

    fn is_win(&self) -> bool {
        for i in 0..self.size {
            let mut row_sum = 0;
            let mut col_sum = 0;
            for j in 0..self.size {
                row_sum += self.data[i][j];
                col_sum += self.data[j][i];
            }
            if row_sum == -5 || col_sum == -5 {
                return true;
            }
        }
        return false;
    }

    fn sum_rest(&self) -> i32 {
        let mut sum = 0;
        for row in &self.data {
            for c in row {
                sum += if *c != -1 { *c } else { 0 };
            }
        }
        return sum;
    }
}

fn new_board(size: usize, lines: Vec<String>) -> Board {
    let mut data = vec![vec![0; size]; size];
    let mut idx = 0;
    for line in lines {
        for n_str in line.split_whitespace() {
            data[idx / size][idx % size] = n_str.parse::<i32>().unwrap();
            idx += 1;
        }
    }
    return Board {
        size,
        data,
        win: false,
    };
}

fn drawn_nums(line: String) -> Vec<i32> {
    let mut nums = vec![];
    for n_str in line.split(",") {
        nums.push(n_str.parse::<i32>().unwrap());
    }
    return nums;
}

fn solve_p1(path: String) -> i32 {
    const BOARD_SIZE: usize = 5;
    let mut lines = utils::read_file_as_strs(path);
    let mut boards: Vec<Board> = vec![];

    let nums = drawn_nums(lines.remove(0));
    for i in (0..lines.len()).step_by(BOARD_SIZE) {
        boards.push(new_board(BOARD_SIZE, lines[i..i + BOARD_SIZE].to_vec()));
    }

    for n in nums {
        for b in boards.iter_mut() {
            b.mark(n);
            if b.is_win() {
                return n * b.sum_rest();
            }
        }
    }

    return 0;
}

fn solve_p2(path: String) -> i32 {
    const BOARD_SIZE: usize = 5;
    let mut lines = utils::read_file_as_strs(path);
    let mut boards: Vec<Board> = vec![];

    let nums = drawn_nums(lines.remove(0));
    for i in (0..lines.len()).step_by(BOARD_SIZE) {
        boards.push(new_board(BOARD_SIZE, lines[i..i + BOARD_SIZE].to_vec()));
    }

    let num_boards = boards.len();
    for n in nums {
        let mut num_wins = 0;
        for b in boards.iter() {
            if b.win {
                num_wins += 1;
            }
        }
        for b in boards.iter_mut() {
            if b.win {
                continue;
            }
            b.mark(n);
            if b.is_win() {
                b.win = true;
                if num_wins == num_boards - 1 {
                    return n * b.sum_rest();
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let tests = [
            (utils::example_path(DAY_NO), 4512),
            (utils::input_path(DAY_NO), 33348),
        ];

        for test in tests {
            assert_eq!(solve_p1(test.0), test.1);
        }
    }

    #[test]
    fn test_p2() {
        let tests = [
            (utils::example_path(DAY_NO), 1924),
            (utils::input_path(DAY_NO), 8112),
        ];

        for test in tests {
            assert_eq!(solve_p2(test.0), test.1);
        }
    }
}

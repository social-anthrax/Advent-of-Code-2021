use crate::task_handler::get_task;
use itertools::Itertools;

trait Adjacent {
    fn get_adjacent(&self, x: usize, y: usize) -> [Option<u32>; 4];
    fn less_than_adjacent(&self, x: usize, y: usize) -> bool;
    fn check_lowest(&self, x: usize, y: usize) -> Option<u32>;
}

impl Adjacent for Vec<Vec<u32>> {
    /// Returns the list of adjacent values (y-1, x+1, y+1, x-1)
    fn get_adjacent(&self, x: usize, y: usize) -> [Option<u32>; 4] {
        let is_valid = |x: usize, y: usize| !(x >= self.len() || y >= self[0].len());

        let mut out: [Option<u32>; 4] = [None; 4];

        if y != 0 && is_valid(x, y - 1) {
            out[0] = Some(self[x][y - 1]);
        };
        if is_valid(x + 1, y) {
            out[1] = Some(self[x + 1][y])
        };
        if is_valid(x, y + 1) {
            out[2] = Some(self[x][y + 1]);
        };
        if x != 0 && is_valid(x - 1, y) {
            out[3] = Some(self[x - 1][y]);
        }

        out
    }
    fn less_than_adjacent(&self, x: usize, y: usize) -> bool {
        let val = self[x][y];
        self.get_adjacent(x, y)
            .iter()
            .filter_map(|&x| x) //Filter map removes all None values.
            .all(|x| val < x)
    }
    fn check_lowest(&self, x: usize, y: usize) -> Option<u32> {
        if self.less_than_adjacent(x, y) {
            Some(self[x][y])
        } else {
            None
        }
    }
}

pub fn task9_1() -> String {
    let input = get_task(9)
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| {
                    x.to_digit(10)
                        .unwrap_or_else(|| panic!("failed on line: {}", &line))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (0..input.len())
        .cartesian_product(0..input[0].len())
        .map(|(x, y)| input.check_lowest(x, y))
        .flatten()
        .map(|x| x + 1)
        .sum::<u32>()
        .to_string()
}

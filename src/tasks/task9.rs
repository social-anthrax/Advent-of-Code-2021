use crate::task_handler::get_task;
use itertools::Itertools;

trait Adjacent {
    fn get_adjacent(&self, x: usize, y: usize) -> [Option<u32>; 4];
    fn less_than_adjacent(&self, x: usize, y: usize) -> bool;
    fn check_lowest(&self, x: usize, y: usize) -> Option<u32>;
    fn is_move_valid(&self, x: usize, y: usize) -> bool;
}

impl Adjacent for Vec<Vec<u32>> {
    fn is_move_valid(&self, x: usize, y: usize) -> bool {
        !(x >= self.len() || y >= self[0].len())
    }

    /// Returns the list of adjacent values (y-1, x+1, y+1, x-1)
    fn get_adjacent(&self, x: usize, y: usize) -> [Option<u32>; 4] {
        let mut out: [Option<u32>; 4] = [None; 4];

        if y != 0 && self.is_move_valid(x, y - 1) {
            out[0] = Some(self[x][y - 1]);
        };
        if self.is_move_valid(x + 1, y) {
            out[1] = Some(self[x + 1][y])
        };
        if self.is_move_valid(x, y + 1) {
            out[2] = Some(self[x][y + 1]);
        };
        if x != 0 && self.is_move_valid(x - 1, y) {
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

fn get_input() -> Vec<Vec<u32>> {
    get_task(9)
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| {
                    x.to_digit(10)
                        .unwrap_or_else(|| panic!("failed on line: {}", &line))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn task9_1() -> String {
    let input = get_input();

    (0..input.len())
        .cartesian_product(0..input[0].len())
        .map(|(x, y)| input.check_lowest(x, y))
        .flatten()
        .map(|x| x + 1)
        .sum::<u32>()
        .to_string()
}

trait SearchFlow {
    fn recursive_search(&self, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> usize;
}

impl SearchFlow for Vec<Vec<u32>> {
    fn recursive_search(&self, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> usize {
        if visited[x][y] || self[x][y] == 9 {
            visited[x][y] = true;
            return 0;
        }

        visited[x][y] = true;

        let mut count = 1;

        if y != 0 && self.is_move_valid(x, y - 1) {
            count += self.recursive_search(visited, x, y - 1);
        };
        if x != 0 && self.is_move_valid(x - 1, y) {
            count += self.recursive_search(visited, x - 1, y);
        };
        if self.is_move_valid(x + 1, y) {
            count += self.recursive_search(visited, x + 1, y);
        };
        if self.is_move_valid(x, y + 1) {
            count += self.recursive_search(visited, x, y + 1);
        };
        count
    }
}

pub fn task9_2() -> String {
    let input = get_input();
    let mut visited = vec![vec![false; input[0].len()]; input.len()];

    let mut top_three = [0; 3];

    (0..input.len())
        .cartesian_product(0..input[0].len())
        .map(|(x, y)| input.recursive_search(&mut visited, x, y))
        //O(n) time :D
        .for_each(|basin| {
            let min = top_three.iter().enumerate().fold(0, |acc, elem| {
                if *elem.1 < top_three[acc] {
                    elem.0
                } else {
                    acc
                }
            });
            if basin > top_three[min] {
                top_three[min] = basin;
            }
        });

    top_three.iter().product::<usize>().to_string()
}

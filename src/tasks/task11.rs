use crate::task_handler::get_task;
use std::fmt;

struct Octopus {
    level: u32,
    flashed: bool,
    total_flashed: usize,
}

impl Octopus {
    // Increments energy level, and returns true if flash.
    // Increments total flash counter if successful.
    // Takes reciever to clear flashed.
    pub fn increment(&mut self) -> bool {
        if self.level >= 9 && !self.flashed {
            self.total_flashed += 1;
            self.level = 0;
            self.flashed = true;
            return true;
        } else if !self.flashed {
            self.level += 1;
        }
        false
    }
}

impl std::fmt::Display for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.level)
    }
}

impl From<char> for Octopus {
    fn from(ch: char) -> Self {
        Octopus {
            level: char::to_digit(ch, 10).unwrap_or_else(|| panic!("Failed to unwrap, {}", ch)),
            flashed: false,
            total_flashed: 0,
        }
    }
}

fn get_valid_near_xy<T>(grid: &[Vec<T>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    if x != 0 {
        out.push((x - 1, y));
    }

    if x < grid.len() - 1 {
        out.push((x + 1, y));
    }

    if y != 0 {
        out.push((x, y - 1));
        if x != 0 {
            out.push((x - 1, y - 1))
        }
        if x < grid.len() - 1 {
            out.push((x + 1, y - 1))
        }
    }

    if y < grid[0].len() - 1 {
        out.push((x, y + 1));
        if x != 0 {
            out.push((x - 1, y + 1))
        }
        if x < grid.len() - 1 {
            out.push((x + 1, y + 1))
        }
    }

    out
}

fn increment_near(grid: &mut Vec<Vec<Octopus>>, x: usize, y: usize) {
    get_valid_near_xy(grid, x, y).iter().for_each(|(x, y)| {
        if grid[*x][*y].increment() {
            increment_near(grid, *x, *y);
        }
    });
}

pub fn task_1() -> String {
    let mut grid = get_task(11)
        .lines()
        .map(|line| line.chars().map(Octopus::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for _ in 0..100 {
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if (grid[x][y]).increment() {
                    increment_near(&mut grid, x, y);
                }
            }
        }

        grid.iter_mut()
            .map(|line| line.iter_mut())
            .flatten()
            .for_each(|x| x.flashed = false);

        // Debug printing
        // grid.iter().for_each(|line| {
        //     line.iter().for_each(|x| print!("{}", x.level));
        //     println!();
        // });
    }

    grid.iter()
        .flatten()
        .fold(0, |accum, elem| accum + elem.total_flashed)
        .to_string()
}

pub fn task_2() -> String {
    let mut grid = get_task(11)
        .lines()
        .map(|line| line.chars().map(Octopus::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0.. {
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if (grid[x][y]).increment() {
                    increment_near(&mut grid, x, y);
                }
            }
        }

        if grid.iter().flatten().all(|o| o.flashed) {
            return (i + 1).to_string();
        }

        grid.iter_mut()
            .map(|line| line.iter_mut())
            .flatten()
            .for_each(|x| x.flashed = false);
    }

    grid.iter()
        .flatten()
        .fold(0, |accum, elem| accum + elem.total_flashed)
        .to_string()
}

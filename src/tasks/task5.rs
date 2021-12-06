use crate::task_handler::get_task;
use ingrid::{coord, size, Coordinate, Grid, Size};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn from_string(input: String) -> Self {
        let input: Vec<usize> = input
            .split(',')
            .map(|x| {
                x.replace(' ', "")
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("Tried to parse {}", x))
            })
            .collect();
        Self {
            x: input[0],
            y: input[1],
        }
    }
}

#[derive(Debug)]
pub struct Line {
    from: Point,
    to: Point,
}

impl Line {
    pub fn from_vec(pair: Vec<Point>) -> Self {
        assert!(pair.len() == 2);
        Self {
            from: pair[0],
            to: pair[1],
        }
    }
}

pub fn task5_2() -> String {
    let input_lines: Vec<Line> = get_task(5)
        .lines()
        .map(|line| {
            line.split("->")
                .map(|x| Point::from_string(x.to_string()))
                .collect()
        })
        .map(Line::from_vec)
        .collect();

    let coordinates: Vec<Point> = get_task(5)
        .lines()
        .map(|line| {
            line.split("->")
                .map(|x| Point::from_string(x.to_string()))
                .collect::<Vec<Point>>()
        })
        .flatten()
        .collect();

    let max_x = coordinates.iter().max_by_key(|f| f.x).unwrap().x as usize;
    let max_y = coordinates.iter().max_by_key(|f| f.y).unwrap().y as usize;

    let mut grid = Grid::<u8>::with_capacity(size!(max_x + 1, max_y + 1));
    grid.resize(size!(max_x + 1, max_y + 1), 0);

    // go through lines

    for line in input_lines {
        let vec: (isize, isize) = (
            (line.to.x as isize - line.from.x as isize),
            (line.to.y as isize - line.from.y as isize),
        );
        let (x_incr, y_incr) = (
            vec.0.checked_div(vec.0.abs()).unwrap_or(0),
            vec.1.checked_div(vec.1.abs()).unwrap_or(0),
        );
        let (mut x, mut y) = (line.from.x, line.from.y);
        grid[coord!(x, y)] += 1;
        while (x, y) != (line.to.x, line.to.y) {
            x = (x as isize + x_incr) as usize;
            y = (y as isize + y_incr) as usize;
            grid[coord!(x, y)] += 1;
        }
    }
    grid.iterator()
        .enumerate()
        .filter(|(_, value)| **value > 1) // as 1 indicates a line, and 2 indicates an overlap
        .count()
        .to_string()
}

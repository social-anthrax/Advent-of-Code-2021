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

pub fn task5_1() -> String {
    let input_lines: Vec<Line> = get_task(5)
        .lines()
        .map(|line| {
            line.split("->")
                .map(|x| Point::from_string(x.to_string()))
                .collect()
        })
        .map(|x| Line::from_vec(x))
        .collect();

    print!("{:#?}", input_lines[0]);
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

    println!("{}, {}", &max_x, &max_y);
    let mut grid = Grid::<u8>::with_capacity(size!(max_x + 1, max_y + 1));
    grid.resize(size!(max_x + 1, max_y + 1), 0);

    drop(max_x);
    drop(max_y);

    for line in input_lines {
        println!(
            "{},{} -> {},{}",
            line.from.x, line.from.y, line.to.x, line.to.y
        );

        if line.from.y == line.to.y {
            let from_x;
            let to_x;

            // as rust needs a<b in a..b
            if line.from.x < line.to.x {
                from_x = line.from.x;
                to_x = line.to.x;
            } else {
                from_x = line.to.x;
                to_x = line.from.x;
            }
            let y = line.from.y;

            for x in (from_x)..(to_x + 1) {
                println!("{},{}", x, y);
                grid[coord!(x, y)] += 1;
            }
        } else if line.from.x == line.to.x {
            let from_y;
            let to_y;

            // as rust needs a<b in a..b
            if line.from.y < line.to.y {
                from_y = line.from.y;
                to_y = line.to.y;
            } else {
                from_y = line.to.y;
                to_y = line.from.y;
            }

            let x = line.from.x;
            for y in from_y..(to_y + 1) {
                println!("{},{}", x, y);
                grid[coord!(x, y)] += 1;
            }
        } 
    }

    for row in grid.rows() {
        row.iterator().for_each(|x| print!("{}", x));
        println!("")
    }

    grid.iterator()
        .enumerate()
        .filter(|(_, value)| **value > 1) // as 1 indicates a line, and 2 indicates an overlap
        .count()
        .to_string()
}

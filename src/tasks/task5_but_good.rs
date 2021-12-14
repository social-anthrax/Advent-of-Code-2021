use crate::task_handler::get_task;
use core::panic;
use regex::Regex;
use std::{cmp, collections::HashMap};

pub fn tasks() -> (String, String) {
    let mut p1 = HashMap::<(usize, usize), usize>::new();
    let mut p2 = HashMap::<(usize, usize), usize>::new();

    for line in get_task(5).lines() {
        //TODO: remove regex
        let (x1, y1, x2, y2) = match Regex::new("\\d+")
            .unwrap()
            .find_iter(line)
            .map(|x| x.as_str().replace(' ', "").parse::<isize>().unwrap())
            .collect::<Vec<isize>>()[..]
        {
            [x1, y1, x2, y2] => (x1, y1, x2, y2),
            _ => panic!("Wrong size"),
        };

        let dx = x2 - x1;
        let dy = y2 - y1;

        for i in 0..(1 + cmp::max(dx.abs(), dy.abs())) {
            let x = x1 + i * dx.signum();
            let y = y1 + i * dy.signum();

            if dx == 0 || dy == 0 {
                *p1.entry((x as usize, y as usize)).or_insert(0) += 1;
            }
            *p2.entry((x as usize, y as usize)).or_insert(0) += 1;
        }
    }

    let count_crossover = |map: HashMap<(usize, usize), usize>| {
        map.iter().filter(|(_, val)| **val > 1).count().to_string()
    };

    (count_crossover(p1), count_crossover(p2))
}

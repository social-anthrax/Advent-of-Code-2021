use std::collections::HashSet;

use crate::task_handler::get_task;

pub fn task8_1() -> String {
    let mut input = get_task(8)
        .lines()
        .map(|x| {
            x.split('|')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .map(|x| (x.chars().collect::<HashSet<char>>()))
                .collect::<Vec<HashSet<char>>>()
        })
        .collect::<Vec<Vec<HashSet<char>>>>();

    input
        .iter_mut()
        .for_each(|f| f.retain(|x| [2, 4, 3, 7].contains(&x.len())));

    input.iter().map(|x| x.len()).sum::<usize>().to_string()
}

fn determine_numbers(input: &[HashSet<char>]) -> [HashSet<char>; 10] {
    //   aaa
    //  b   c
    //  b   c
    //   ddd
    //  e   f
    //  e   f
    //   ggg

    let one = input.iter().find(|x| x.len() == 2).unwrap();
    let four = input.iter().find(|x| x.len() == 4).unwrap();
    let seven = input.iter().find(|x| x.len() == 3).unwrap();
    let eight = input.iter().find(|x| x.len() == 7).unwrap();
    let six = input
        .iter()
        .find(|x| x.len() == 6 && one.difference(x).collect::<HashSet<_>>().len() == 1)
        .unwrap();
    let c = eight.difference(six).cloned().collect();
    let five = input
        .iter()
        .find(|x| x.len() == 5 && x.is_disjoint(&c))
        .unwrap_or_else(|| panic!("unwrap failed when trying to find a value x that would have a difference of 1 with {:?} and x being disjoint with {:?}", six, c));

    let nine: HashSet<char> = five.union(&c).cloned().collect();

    let zero = input
        .iter()
        .find(|&x| x.len() == 6 && x.ne(six) && x.ne(&nine))
        .unwrap();

    let three = input
        .iter()
        .find(|&x| x.len() == 5 && one.is_subset(x))
        .unwrap();
    let two = input
        .iter()
        .find(|&x| x.len() == 5 && x.ne(three) && x.ne(five))
        .unwrap();

    [
        zero.clone(),
        one.clone(),
        two.clone(),
        three.clone(),
        four.clone(),
        five.clone(),
        six.clone(),
        seven.clone(),
        eight.clone(),
        nine,
    ]
}

pub fn task8_2() -> String {
    let inputs = get_task(8)
        .lines()
        .map(|x| x.split('|').next().unwrap())
        .map(|s| {
            s.trim()
                .split_whitespace()
                .map(|x| HashSet::from_iter(x.chars()))
                .collect::<Vec<HashSet<char>>>()
        })
        .collect::<Vec<_>>();

    let outputs = get_task(8)
        .lines()
        .map(|s| s.split(" | ").nth(1).unwrap())
        .map(|s| {
            s.trim()
                .split_whitespace()
                .map(|x| HashSet::from_iter(x.chars()))
                .collect::<Vec<HashSet<char>>>()
        })
        .collect::<Vec<_>>();

    inputs
        .iter()
        .zip(outputs.iter())
        // For each line of input, calculate the numbers, and then fold the four output numbers.
        .map(|(input, output)| {
            let numbers = determine_numbers(input);
            output.iter().fold(0, |acc, value| {
                let idx = numbers.iter().position(|x| x.eq(value)).unwrap_or_else(|| {
                    panic!(
                        "Failed to unwrap at value: {:?} \n Numbers: {:#?}",
                        value, numbers
                    )
                });
                acc * 10 + idx
            })
        })
        .sum::<usize>()
        .to_string()
}

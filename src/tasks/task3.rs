use std::usize;

use crate::task_handler::get_task;

fn most_common_bit(numbers: &Vec<usize>) -> usize {
    let mut gamma: usize = 0;
    for column in 0..12 {
        let (one, zero): (Vec<usize>, Vec<usize>) = numbers
            .iter()
            .map(|x| (x >> column) & 1)
            .partition(|x: &usize| *x == 1);

        if one.len() > zero.len() {
            gamma |= 1 << column;
        }
    }
    gamma
}

fn most_common_single_bit(column: usize, numbers: &Vec<usize>) -> usize {
    let (one, zero): (Vec<usize>, Vec<usize>) = numbers
        .iter()
        .map(|x| (x >> column) & 1)
        .partition(|x: &usize| *x == 1);

    if one.len() >= zero.len() {
        1
    } else {
        0
    }
}

pub fn task3_1() -> String {
    let input: Vec<usize> = get_task(3)
        .lines()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect();
    let gamma: usize = most_common_bit(&input);
    let epsilon = !gamma & 0b111111111111;
    (gamma as u64 * epsilon as u64).to_string() // as the inverse of gamma should be epsilon
}

pub fn task3_2() -> String {
    let input: Vec<usize> = get_task(3)
        .lines()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect();

    let mut ox_input = input.clone();
    let oxygen_rating = {
        let mut common_bit;
        for bit in (0..12).rev() {
            if ox_input.len() == 1 {
                break;
            }
            common_bit = most_common_single_bit(bit, &ox_input);
            ox_input.retain(|x| (*x >> bit) & 1 == common_bit);
        }

        ox_input[0]
    };

    let mut co2_input = input.clone();
    let co2_rating = {
        let mut common_bit: usize;
        for bit in (0..12).rev() {
            if co2_input.len() == 1 {
                break;
            }
            common_bit = !most_common_single_bit(bit, &co2_input) & 1;
            co2_input.retain(|x| (*x >> bit) & 1 == common_bit);
        }

        co2_input[0]
    };

    (co2_rating * oxygen_rating).to_string()
}

use crate::task_handler::get_task;

fn most_common_bit(numbers: &[usize]) -> usize {
    let mut gamma: usize = 0;
    (0..12).for_each(|column| {
        let (one_vec, zero_vec): (Vec<usize>, Vec<usize>) = numbers
            .iter()
            .map(|x| (x >> column) & 1)
            .partition(|x: &usize| *x == 1);

        if one_vec.len() > zero_vec.len() {
            gamma |= 1 << column;
        }
    });
    gamma
}

fn most_common_single_bit(column: usize, numbers: &[usize]) -> usize {
    let (one, zero): (Vec<usize>, Vec<usize>) = numbers
        .iter()
        .map(|x| (x >> column) & 1)
        .partition(|x: &usize| *x == 1);

    (one.len() >= zero.len()) as usize
}

pub fn task3_1_cursed() -> String {
    let input: Vec<usize> = get_task(3)
        .lines()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect();
    let gamma: usize = most_common_bit(&input);
    let epsilon = !gamma & 0b1111_1111_1111;
    (gamma as u64 * epsilon as u64).to_string() // as the inverse of gamma should be epsilon
}

pub enum InputType {
    Oxygen,
    CO2,
}

pub fn task3_2_cursed() -> String {
    let input: Vec<usize> = get_task(3)
        .lines()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect();

    let calculate = |input: &Vec<usize>, input_type: InputType| -> usize {
        let mut input = input.clone();

        (0..12).rev().try_for_each(|y| {
            if input.len() == 1 {
                return None;
            }
            let common_bit: usize = match input_type {
                InputType::Oxygen => most_common_single_bit(y, &input),
                InputType::CO2 => !most_common_single_bit(y, &input) & 1,
            };

            input.retain(|x| (*x >> y) & 1 == common_bit);
            return Some(());
        });

        return input[0];
    };

    (calculate(&input, InputType::CO2) * calculate(&input, InputType::Oxygen)).to_string()
}

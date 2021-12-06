use crate::task_handler::get_task;

pub fn task6_input() -> Vec<usize> {
    let mut input = get_task(6);
    input.retain(|c| !c.is_whitespace());
    input
        .split(',')
        .map(|x| {
            x.parse::<usize>()
                .unwrap_or_else(|_| panic!("Failed at following input: '{}'", x))
        })
        .collect()
}

fn cycle(age_count: &mut [usize]) {
    age_count.rotate_left(1);
    age_count[6] += age_count[8];
}

fn count_at_days(data: &[usize], time_period: usize) -> usize {
    let mut ages = [0; 9];
    for &fish_age in data {
        ages[fish_age] += 1;
    }
    (0..time_period).for_each(|_| cycle(&mut ages));
    ages.iter().sum()
}

pub fn task6_1() -> String {
    count_at_days(&task6_input(), 80).to_string()
}

pub fn task6_2() -> String {
    count_at_days(&task6_input(), 256 ).to_string()
}

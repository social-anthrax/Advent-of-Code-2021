use crate::task_handler::get_task;
pub fn task_1() -> String {
    let input: Vec<usize> = get_task(1)
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    input.windows(2).filter(|x| x[0] < x[1]).count().to_string()
}

pub fn task_2() -> String {
    let input: Vec<usize> = get_task(1)
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    input
        .windows(3)
        .collect::<Vec<&[usize]>>()
        .windows(2)
        .filter(|x| x[0].iter().sum::<usize>() < x[1].iter().sum())
        .count()
        .to_string()
}

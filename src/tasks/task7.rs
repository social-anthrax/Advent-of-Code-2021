use crate::quickselect::nth_element;
use crate::task_handler::get_task;

fn input() -> Vec<usize> {
    get_task(7)
        .replace(' ', "")
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|x| {
            x.parse::<usize>()
                .unwrap_or_else(|_| panic!("Panicked at value {}", x))
        })
        .collect()
}
pub fn task7_1() -> String {
    let mut input = input();

    // get median
    let i = input.len() / 2;
    let median = nth_element(&mut input, i);
    let mse: f64 = input
        .iter()
        .map(|&x| (x as isize - median as isize).abs() as usize)
        .sum::<usize>() as f64;
    mse.to_string()
}

pub fn task7_2() -> String {
    // This totally abuses my friends maths. Very impressive, go take a look.
    // https://yellowtid.es/blog/posts/advent-of-code-2021-day-7-minimising-mse-aad/
    let input = input();
    let mean_unround = input.iter().sum::<usize>() as f64 / input.len() as f64;

    let aad_plus_mse = |input: &Vec<usize>, x_i: usize| -> f64 {
        let c = 1.0 / input.len() as f64;
        c * input
            .iter()
            .map(|&x| (x as isize - x_i as isize).pow(2) as f64)
            .sum::<f64>()
            + c * input
                .iter()
                .map(|&x| (x as isize - x_i as isize).abs() as f64)
                .sum::<f64>()
    };

    let lower = aad_plus_mse(&input, mean_unround.floor() as usize);
    let upper = aad_plus_mse(&input, mean_unround.ceil() as usize);
    if lower > upper {
        (0.5 * input.len() as f64 * upper).to_string()
    } else {
        (0.5 * input.len() as f64 * lower).to_string()
    }
}

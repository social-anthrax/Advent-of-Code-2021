use crate::task_handler::get_task;

pub fn task2_1() -> String {
    let input = get_task(2);
    let mut depth = 0;
    let mut horizontal = 0;

    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["forward", x] => horizontal += x.parse::<i32>().unwrap(),
            ["down", x] => depth += x.parse::<i32>().unwrap(),
            ["up", x] => depth -= x.parse::<i32>().unwrap(),
            _ => panic!("Unrecognised instruction"),
        }
    }

    (depth * horizontal).to_string()
}

pub fn task2_2() -> String {
    let input = get_task(2);
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["forward", x] => {
                horizontal += x.parse::<i32>().unwrap();
                depth += aim * x.parse::<i32>().unwrap();
            }
            ["down", x] => aim += x.parse::<i32>().unwrap(),
            ["up", x] => aim -= x.parse::<i32>().unwrap(),
            _ => panic!("Unrecognised instruction"),
        }
    }

    (depth * horizontal).to_string()
}

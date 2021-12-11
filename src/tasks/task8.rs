use crate::task_handler::get_task;

fn task8_1() -> String {
    let input = get_task(8).lines().map(|x| x.split('|').nth(1).unwrap().split(' ').collect());
    todo!()
}

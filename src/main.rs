mod task_handler;
mod tasks;
use tasks::*;

fn main() {
    println!("Task1.1 {}", task1::task_1());
    println!("Task1.2 {}", task1::task_2());
    println!("Task2.1 {}", task2::task_1());
    println!("Task2.2 {}", task2::task_2());
    println!("Task3.1 {}", task3::task_1());
    println!("Task3.2 {}", task3::task_2());
    println!("Task3.1 cursed {}", task3_cursed::task_1_cursed());
    println!("Task3.2 cursed {}", task3_cursed::task_2_cursed());
    println!("Task4.1 cursed {}", task4::task_1());
    println!("Task4.2 cursed {}", task4::task_2());
    println!("Task5.1 {}", task5::task_2());
    let (neat_1, neat_2) = task5_but_good::tasks();

    println!("Task5.1 {} \n Task5.2 {}", neat_1, neat_2);
    println!("Task6.1 {}", task6::task_1());
    println!("Task6.2 {}", task6::task_2());
    println!("Task7.1 {}", task7::task_1());
    println!("Task7.2 {}", task7::task_2());
    println!("Task8.1 {}", task8::task_1());
    println!("Task8.2 {}", task8::task_2());
    println!("Task9.1 {}", task9::task_1());
    println!("Task9.2 {}", task9::task_2());
    println!("Task10.1 {}", task10::task_1());
    println!("Task10.2 {}", task10::task_2());
    println!("Task11.1 {}", task11::task_1());
    println!("Task11.2 {}", task11::task_2())
}

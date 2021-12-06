mod task_handler;
mod tasks;
use tasks::*;

use crate::tasks::{task3_cursed::{task3_1_cursed, task3_2_cursed}, task6::{task6_1, task6_2}};
fn main() {
    println!("Task1.1 {}", task1::task1_1());
    println!("Task1.2 {}", task1::task1_2());
    println!("Task2.1 {}", task2::task2_1());
    println!("Task2.2 {}", task2::task2_2());
    println!("Task3.1 {}", task3::task3_1());
    println!("Task3.2 {}", task3::task3_2());
    println!("Task3.1 cursed {}", task3_1_cursed());
    println!("Task3.2 cursed {}", task3_2_cursed());
    println!("Task4.1 cursed {}", task4::task4_1());
    println!("Task4.2 cursed {}", task4::task4_2());
    println!("Task5.1 {}", task5::task5_2());
    let (neat_1, neat_2) = task5_but_good::task5();

    println!("Task5.1 {} \nTask5.2 {}", neat_1, neat_2);
    println!("Task6.1 {}" , task6_1());
    println!("Task6.2 {}" , task6_2());
}

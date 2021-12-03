mod task_handler;
mod tasks;
use tasks::*;

use crate::tasks::task3_cursed::{task3_1_cursed, task3_2_cursed};
fn main() {
    println!("Task1.1 {}", task1::task1_1());
    println!("Task1.2 {}", task1::task1_2());
    println!("Task2.1 {}", task2::task2_1());
    println!("Task2.2 {}", task2::task2_2());
    println!("Task3.1 {}", task3::task3_1());
    println!("Task3.2 {}", task3::task3_2());
    println!("Task3.1 cursed {}", task3_1_cursed());
    println!("Task3.2 cursed {}", task3_2_cursed());
}

use std::{collections::HashMap, io};
use uuid::Uuid;

mod task;
use crate::task::Task;

fn main() {
    let mut todos: HashMap<Uuid, Task> = HashMap::new();
    println!("Welcome to todo list system");

    loop {
        println!("=== To-Do List Manager ===");
        println!("1. Add a Task");
        println!("2. View Tasks");
        println!("3. Search Tasks");
        println!("4. Update Tasks");
        println!("5. Delete Tasks");
        println!("6. Exit Program");
        println!("Enter choice:");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
    }
}

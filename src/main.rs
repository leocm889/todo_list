use std::{collections::HashMap, io};
use uuid::Uuid;

mod priority;
mod status;
mod todo;
use crate::todo::{add_todo, delete_todo, retrieve_todos_sorted, search_menu, update_todo, Todo};

fn main() {
    let mut todos: HashMap<Uuid, Todo> = HashMap::new();
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

        match choice {
            1 => add_todo(&mut todos),
            2 => retrieve_todos_sorted(&todos),
            3 => search_menu(&todos),
            4 => update_todo(&mut todos),
            5 => delete_todo(&mut todos),
            6 => {
                println!("Goodbye! 👋");
                break;
            }
            _ => {
                println!("Invalid choice, try again.");
                continue;
            }
        }
    }
}

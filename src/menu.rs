use crate::todo::{add_todo, delete_todo, retrieve_todos_sorted, search_menu, update_todo};
use colored::*;
use std::io;

pub fn display_menu(file_path: &str) {
    println!("{}", "🎉 Welcome to the To-Do list System!".bold().blue());

    loop {
        println!("{}", "=== To-Do List Manager ===".bold().cyan());
        println!("{}", "1. ➕ Add a Task".green());
        println!("{}", "2. 📋 View Tasks".yellow());
        println!("{}", "3. 🔍 Search Tasks".magenta());
        println!("{}", "4. ✏️ Update Tasks".blue());
        println!("{}", "5. 🗑 Delete Tasks".red());
        println!("{}", "6. 🚪 Exit Program".white().bold());
        println!("{}", "👉 Enter choice:".bold());

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("❌ Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⚠️ Please enter a valid number".yellow().bold());
                continue;
            }
        };

        match choice {
            1 => add_todo(file_path),
            2 => retrieve_todos_sorted(file_path),
            3 => search_menu(file_path),
            4 => update_todo(file_path),
            5 => delete_todo(file_path),
            6 => {
                println!("{}", "👋 Goodbye!".bold().green());
                break;
            }
            _ => {
                println!("{}", "❌ Invalid choice, try again.".red().bold());
                continue;
            }
        }
    }
}

use clap::Parser;
mod cli;
mod menu;
mod priority;
mod status;
mod storage;
mod todo;
mod todo_cli;
mod utils;
use menu::display_menu;
use uuid::Uuid;

use crate::cli::{Cli, Commands};
use crate::todo_cli::{
    add_todo_cli, delete_todo_cli, list_todos_cli, search_todo_cli, update_todo_cli,
};
use crate::utils::{parse_priority, parse_status};

fn main() {
    let file_path = "todos.json";
    let cli = Cli::parse();

    if cli.menu {
        display_menu(file_path);
        return;
    }

    match cli.command {
        Some(Commands::Add {
            title,
            description,
            priority,
            status,
        }) => {
            let priority = parse_priority(&priority);
            let status = parse_status(&status);

            add_todo_cli(file_path, title, description, priority, status);
        }
        Some(Commands::List { sort_by }) => {
            list_todos_cli(file_path, &sort_by);
        }
        Some(Commands::Search {
            id,
            title,
            priority,
            status,
        }) => {
            search_todo_cli(file_path, id, title, priority, status);
        }
        Some(Commands::Update {
            id,
            title,
            description,
            priority,
            status,
        }) => {
            let id = Uuid::parse_str(&id).expect("Invalid UUID");
            let priority = priority.map(|p| parse_priority(&p));
            let status = status.map(|s| parse_status(&s));

            if update_todo_cli(file_path, id, title, description, priority, status) {
                println!("✅ Task updated");
            } else {
                println!("⚠️ No task found with id {id}");
            }
        }
        Some(Commands::Delete { id }) => {
            let id = Uuid::parse_str(&id).expect("Invalid UUID");
            if delete_todo_cli(file_path, id) {
                println!("🗑️ Task deleted");
            } else {
                println!("⚠️ No task found with id {id}");
            }
        }
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }
}

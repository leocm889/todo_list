use std::io;

use clap::{CommandFactory, Parser};
use clap_complete::{
    generate,
    shells::{Bash, Fish, PowerShell, Zsh},
};
mod cli;
mod menu;
mod priority;
mod sortby;
mod status;
mod storage;
mod todo;
mod todo_cli;
use colored::*;
use menu::display_menu;
use uuid::Uuid;

use crate::cli::{Cli, Commands, Shell};
use crate::todo_cli::{
    add_todo_cli, delete_todo_cli, list_todos_cli, search_todo_cli, update_todo_cli,
};
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
            let id = Uuid::parse_str(&id).expect("❌ Invalid UUID".red().to_string().as_str());

            if update_todo_cli(file_path, id, title, description, priority, status) {
                println!("{}", "✅ Task updated successfully!".green().bold());
            } else {
                println!(
                    "{}",
                    format!("⚠️ No task found with id {id}").yellow().bold()
                );
            }
        }
        Some(Commands::Delete { id }) => {
            let id = Uuid::parse_str(&id).expect("❌ Invalid UUID");
            if delete_todo_cli(file_path, id) {
                println!("{}", "🗑️ Task deleted".red().bold());
            } else {
                println!(
                    "{}",
                    format!("⚠️ No task found with id {id}").yellow().bold()
                );
            }
        }
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            match shell {
                Shell::Bash => generate(Bash, &mut cmd, "todo", &mut io::stdout()),
                Shell::Zsh => generate(Zsh, &mut cmd, "todo", &mut io::stdout()),
                Shell::Fish => generate(Fish, &mut cmd, "todo", &mut io::stdout()),
                Shell::Powershell => generate(PowerShell, &mut cmd, "todo", &mut io::stdout()),
            }
        }
        None => {
            println!(
                "{}",
                "⚠️ No command provided. Use --help for usage."
                    .yellow()
                    .bold()
            );
        }
    }
}

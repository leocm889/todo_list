use std::io;

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use clap::{CommandFactory, Parser};
use clap_complete::{
    generate,
    shells::{Bash, Fish, PowerShell, Zsh},
};
mod cli;
mod input;
mod menu;
mod priority;
mod recurrence;
mod sortby;
mod status;
mod storage;
mod todo;
mod todo_cli;
mod utils;
use colored::*;
use menu::display_menu;
use uuid::Uuid;

use crate::{
    cli::{Cli, Commands, Shell},
    input::UpdateTodoInput,
};
use crate::{
    input::AddTodoInput,
    todo_cli::{add_todo_cli, delete_todo_cli, list_todos_cli, search_todo_cli, update_todo_cli},
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
            due_date,
            recurrence,
            tags,
            parent_id,
            subtasks,
        }) => {
            let due_date = due_date.map(|d| {
                NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                    .map(|nd| Utc.from_utc_date(&nd).and_hms_opt(0, 0, 0).unwrap())
                    .unwrap_or_else(|_| {
                        panic!(
                            "{}",
                            format!("❌ Invalid date format: {d} (expected YYY-MM-DD)").red()
                        )
                    })
            });

            let parent_id =
                parent_id.map(|pid| Uuid::parse_str(&pid).expect("❌ Invalid parent UUID"));

            let subtasks = subtasks.map(|subs| {
                subs.into_iter()
                    .map(|s| Uuid::parse_str(&s).expect("❌ Invalid subtask UUID"))
                    .collect()
            });

            add_todo_cli(
                file_path,
                AddTodoInput {
                    title,
                    description,
                    priority,
                    status,
                    due_date,
                    tags,
                    recurrence,
                    parent_id,
                    subtasks,
                },
            );
        }
        Some(Commands::List { sort_by }) => {
            list_todos_cli(file_path, &sort_by);
        }
        Some(Commands::Search {
            id,
            title,
            priority,
            status,
            due_date,
            recurrence,
            tags,
            parent_task_id,
        }) => {
            let due_date = due_date.map(|d| {
                NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                    .map(|nd| Utc.from_utc_date(&nd).and_hms_opt(0, 0, 0).unwrap())
                    .unwrap_or_else(|_| {
                        panic!(
                            "{}",
                            format!("❌ Invalid date format: {d} (expected YYY-MM-DD)").red()
                        )
                    })
            });

            let parent_id =
                parent_task_id.map(|pid| Uuid::parse_str(&pid).expect("❌ Invalid parent UUID"));

            search_todo_cli(
                file_path,
                input::SearchTodoInput {
                    id,
                    title,
                    priority,
                    status,
                    due_date,
                    recurrence,
                    tags,
                    parent_id,
                },
            );
        }
        Some(Commands::Update {
            id,
            title,
            description,
            priority,
            status,
            due_date,
            recurrence,
            tags,
            parent_task_id,
            subtasks,
        }) => {
            let id = match Uuid::parse_str(&id) {
                Ok(u) => u,
                Err(_) => {
                    eprintln!("{}", "❌ Invalid UUID".red());
                    std::process::exit(1);
                }
            };

            let due_date = due_date.map(|d| {
                let nd = NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap_or_else(|_| {
                    panic!(
                        "{}",
                        format!("❌ Invalid date format: {d} (expected YYY-MM-DD)").red()
                    )
                });
                let ndt = nd
                    .and_hms_opt(0, 0, 0)
                    .unwrap_or_else(|| panic!("{}", "❌  Invalid time components".red()));
                DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc)
            });

            let parent_id =
                parent_task_id.map(|pid| Uuid::parse_str(&pid).expect("❌ Invalid parent UUID"));

            let parsed_subtasks = subtasks.map(|subs| {
                subs.into_iter()
                    .filter_map(|s| {
                        Uuid::parse_str(&s)
                            .inspect_err(|_| {
                                eprintln!(
                                    "{}",
                                    format!("Invalid UUID format in subtasks: {s}")
                                        .yellow()
                                        .bold()
                                );
                            })
                            .ok()
                    })
                    .collect::<Vec<Uuid>>()
            });

            if update_todo_cli(
                file_path,
                UpdateTodoInput {
                    id,
                    new_title: title,
                    new_description: description,
                    new_priority: priority,
                    new_status: status,
                    new_due_date: due_date,
                    new_recurrence: recurrence,
                    new_tags: tags,
                    new_parent_id: parent_id,
                    new_subtasks: parsed_subtasks,
                },
            ) {
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

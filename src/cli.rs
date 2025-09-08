use crate::{priority::Priority, sortby::SortBy, status::Status};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "todo",
    about = "A simple todo program with CLI or menu mode",
    long_about = "Manage your tasks via a command-line interface or interactive menu"
)]
pub struct Cli {
    #[arg(long, help = "RUn in interactive menu mode")]
    pub menu: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(help = "Short title for your task")]
        title: String,
        #[arg(help = "Optional longer description")]
        description: String,
        #[arg(
            long,
            default_value = "medium",
            value_enum,
            help = "Set task priority (low, medium, high)"
        )]
        priority: Priority,
        #[arg(
            long,
            default_value = "pending",
            value_enum,
            help = "Set initial status of the task (pending, inprogress, done)"
        )]
        status: Status,
    },
    List {
        #[arg(
            long,
            default_value = "created",
            value_enum,
            help = "Choose how to sort tasks (priority, status, created)"
        )]
        sort_by: SortBy,
    },
    Search {
        #[arg(long, help = "Find task by its unique ID")]
        id: Option<String>,
        #[arg(long, help = "Find tasks by title (partial match allowed)")]
        title: Option<String>,
        #[arg(long, value_enum, help = "Find tasks by priority")]
        priority: Option<Priority>,
        #[arg(long, value_enum, help = "Find tasks by status")]
        status: Option<Status>,
    },
    Update {
        #[arg(help = "UUID of the task you want to modify")]
        id: String,
        #[arg(long, help = "Update the task title")]
        title: Option<String>,
        #[arg(long, help = "Update the task description")]
        description: Option<String>,
        #[arg(long, value_enum, help = "Update the task priority")]
        priority: Option<Priority>,
        #[arg(long, value_enum, help = "Update the task status")]
        status: Option<Status>,
    },
    Delete {
        #[arg(help = "UUID of the task to remove")]
        id: String,
    },
    Completions {
        #[arg(value_enum, help = "Choose the shell to generate completions for")]
        shell: Shell,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
}

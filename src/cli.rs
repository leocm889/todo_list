use crate::{priority::Priority, recurrence::Recurrence, sortby::SortBy, status::Status};
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
        #[arg(short = 't', long, help = "Short title for your task")]
        title: String,
        #[arg(short = 'd', long, help = "Optional longer description")]
        description: String,
        #[arg(
            short = 'p',
            long,
            default_value = "medium",
            value_enum,
            help = "Set task priority"
        )]
        priority: Priority,
        #[arg(
            short = 's',
            long,
            default_value = "pending",
            value_enum,
            help = "Set initial status of the task"
        )]
        status: Status,
        #[arg(long, value_enum, help = "Optional recurrence")]
        recurrence: Option<Recurrence>,
        #[arg(
            long,
            value_enum,
            help = "Custom recurrence rule (only used if --recurrence custom)"
        )]
        custom_rule: Option<String>,
    },
    List {
        #[arg(
            short = 's',
            long,
            default_value = "created",
            value_enum,
            help = "Choose how to sort tasks"
        )]
        sort_by: SortBy,
    },
    Search {
        #[arg(short = 'i', long, help = "Find task by its unique ID")]
        id: Option<String>,
        #[arg(
            short = 't',
            long,
            help = "Find tasks by title (partial match allowed)"
        )]
        title: Option<String>,
        #[arg(short = 'p', long, value_enum, help = "Find tasks by priority")]
        priority: Option<Priority>,
        #[arg(short = 's', long, value_enum, help = "Find tasks by status")]
        status: Option<Status>,
    },
    Update {
        #[arg(short = 'i', long, help = "UUID of the task you want to modify")]
        id: String,
        #[arg(short = 't', long, help = "Update the task title")]
        title: Option<String>,
        #[arg(short = 'd', long, help = "Update the task description")]
        description: Option<String>,
        #[arg(short = 'p', long, value_enum, help = "Update the task priority")]
        priority: Option<Priority>,
        #[arg(short = 's', long, value_enum, help = "Update the task status")]
        status: Option<Status>,
    },
    Delete {
        #[arg(short = 'i', long, help = "UUID of the task to remove")]
        id: String,
    },
    Completions {
        #[arg(
            short = 's',
            long,
            value_enum,
            help = "Choose the shell to generate completions for"
        )]
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

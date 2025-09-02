use clap::{Parser, Subcommand};
mod menu;
mod priority;
mod status;
mod todo;
use menu::display_menu;

fn main() {
    let file_path = "todos.json";
    display_menu(file_path);
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo shell program", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
        description: String,
        #[arg(long, default_value = "medium")]
        priority: String,
        #[arg(long, default_value = "pending")]
        status: String,
    },
    List,
}

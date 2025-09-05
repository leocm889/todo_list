use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo program with CLI or menu mode", long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub menu: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,
        description: String,
        #[arg(long, default_value = "medium")]
        priority: String,
        #[arg(long, default_value = "pending")]
        status: String,
    },
    List {
        #[arg(long, default_value = "created")]
        sort_by: String,
    },
    Search {
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    Update {
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    Delete {
        id: String,
    },
}

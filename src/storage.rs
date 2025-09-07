use crate::todo::Todo;
use colored::*;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};
use uuid::Uuid;

pub fn load_todos_from_file(file_path: &str) -> HashMap<Uuid, Todo> {
    match fs::read_to_string(file_path) {
        Ok(data) => match serde_json::from_str::<HashMap<Uuid, Todo>>(&data) {
            Ok(todos) => {
                println!(
                    "{}",
                    format!("✅ Todos loaded successfully from {file_path}")
                        .green()
                        .bold()
                );
                todos
            }
            Err(error) => {
                eprintln!(
                    "{}",
                    format!("❌ Failed to parse JSON: {error}").red().bold()
                );
                HashMap::new()
            }
        },
        Err(_) => {
            println!(
                "{}",
                "⚠️ File not found, starting with an empty list of todos."
                    .yellow()
                    .bold()
            );
            HashMap::new()
        }
    }
}

pub fn save_todos_to_file(todos: &HashMap<Uuid, Todo>, file_path: &str) {
    match File::create(file_path) {
        Ok(mut file) => {
            let json = serde_json::to_string_pretty(todos).unwrap();
            if let Err(error) = file.write_all(json.as_bytes()) {
                eprintln!(
                    "{}",
                    format!("❌ Failed to write to file: {error}").red().bold()
                );
            } else {
                println!(
                    "{}",
                    format!("💾 Todos saved successfully to {file_path}")
                        .green()
                        .bold()
                );
            }
        }
        Err(error) => eprintln!(
            "{}",
            format!("❌ Failed to create file: {error}").red().bold()
        ),
    }
}

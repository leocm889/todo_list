use crate::todo::Todo;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};
use uuid::Uuid;

pub fn save_todos_to_file(todos: &HashMap<Uuid, Todo>, file_path: &str) {
    match File::create(file_path) {
        Ok(mut file) => {
            let json = serde_json::to_string_pretty(todos).unwrap();
            if let Err(error) = file.write_all(json.as_bytes()) {
                eprintln!("Failed to write to file: {error}");
            } else {
                println!("Todos saved successfully to {file_path}");
            }
        }
        Err(error) => eprintln!("Failed to create file: {error}"),
    }
}

pub fn load_todos_from_file(file_path: &str) -> HashMap<Uuid, Todo> {
    match fs::read_to_string(file_path) {
        Ok(data) => match serde_json::from_str::<HashMap<Uuid, Todo>>(&data) {
            Ok(todos) => {
                println!("Todos loaded successfully from {file_path}");
                todos
            }
            Err(error) => {
                eprintln!("Failed to parse JSON: {error}");
                HashMap::new()
            }
        },
        Err(_) => {
            println!("File not found, starting with an empty list of todos.");
            HashMap::new()
        }
    }
}

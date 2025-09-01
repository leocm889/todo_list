mod menu;
mod priority;
mod status;
mod todo;
use menu::display_menu;

fn main() {
    let file_path = "todos.json";
    display_menu(file_path);
}

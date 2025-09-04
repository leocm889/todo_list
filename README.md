# 📝 Todo CLI

A simple "Todo List Manager" written in Rust.
Supports **interactive mode** and **command-line mode** for maximum flexibility.

---

## ✨ Features

-- Add, view, update, search, and delete todos
- Store todos in a JSON file (`todos.json`)
- Interactive menu mode (`--menu`) for guided use
- Command-line interface (CLI) for quick commands
- Todos have:
  - Unique ID (UUID)
  - Title & description
  - Priority (High, Medium, Low)
  - Status (Pending, In Progress, Done)
  - Creation timestap

---

## 🚀 Installation

Clone the repository and install the binary locally:

```bash
git clone https://github.com/leocm889/todo_list.git
cd todo_list
cargo install --path .
```

This will install the binary to `~/.cargo/bin/todo`
Make sure `~/.cargo/bin` is in your `$PATH`

--- 

## 🖥️ Usage

### 1. Interactive Menu Mode

Launch the program with a text-based menu:

```bash
todo --menu
# or while developing
cargo run -- --menu
```

You'll see options like:

```bash
1. Add a Task
2. View Tasks
3. Search Tasks
4. Update Tasks
5. Delete Tasks
6. Exit Program
```

### 2. Command-Line Mode

Use direct commands without the menu:

**Add a todo**

```bash
todo add "Buy milk" "From the store" --priority high --status pending
```

**View todos**

```bash
todo list
```

**Search by title**

```bash
todo search --title "milk"
```

**Search by ID**

```bash
todo search --id <UUID>
```

**Search by priority**

```bash
todo search --priority high
```

**Search by status**

```bash
todo search --status done
```

**Update a todo**

```bash
todo update <UUID> -- title "Buy bread" --status in_progress
```

**Delete a todo

```bash
todo delete <UUID>
```

---

## 📂 Project Structure

```bash
src/
├── main.rs         # CLI entry point -> parses args (clap) and dispatches. Handles --menu flag.
├── lib.rs          # Core library: re-exports + pure logic for tests and shared behavior.
├── cli.rs          # CLI helpers and CLI parsing helpers (optional: move clap config here)
├── menu.rs         # Interactive menu UI; uses interactive functions or calls todo_cli wrappers.
├── todo.rs         # Todo struct + interactive (stdin-based) versions of functions (used by menu).
├── todo_cli.rs     # Non-interactive CLI wrappers (add_todo_cli, update_todo_cli, delete_todo_cli).
├── storage.rs      # Persistence layer: load/save JSON from/to disk.
├── priority.rs     # Priority enum + (optionally) FromStr impl
├── status.rs       # Status enum + (optionally) FromStr impl
├── utils.rs        # small helpers (read_input, read_optional_input, validation helpers)
└── tests/          # integration or unit tests (or you can keep tests inside lib.rs)
```

---

## 📊 Example JSON Output

```json
{
  "b5a9a9c2-6f5a-4b2e-91e1-8e44c0d6c123": {
    "id": "b5a9a9c2-6f5a-4b2e-91e1-8e44c0d6c123",
    "title": "Buy milk",
    "description": "From the supermarket",
    "priority": "High",
    "status": "Pending",
    "created_at": "2025-09-04T12:34:56"
  },
  "e1c2a3f4-1d2e-4c5b-8f6a-7d8c9b0e1234": {
    "id": "e1c2a3f4-1d2e-4c5b-8f6a-7d8c9b0e1234",
    "title": "Finish Rust project",
    "description": "Work on todo-cli",
    "priority": "Medium",
    "status": "InProgress",
    "created_at": "2025-09-04T13:10:42"
  }
}
```

---

## 📌 TODOs / Future Improvements

- Add due dates & reminders

- Support for multiple storage backends (SQLite, PostgreSQL)

- Export todos as CSV/Markdown

- More advanced search/filtering

- Colored output for better readability

---

## ✍️ Author

- leocm889

# 📝 Todo CLI

A simple "Todo List Manager" written in Rust.
Supports both an **interactive menu** and a **command-line interface (CLI)**.

---

## ✨ Features

- Add, list, update, search, and delete todos
- Store todos in a JSON file (`todos.json`)
- Interactive menu mode (`--menu`) for guided use
- CLI for quick commands
- Desktop notifications for due/overdue tasks (`todo notify`)
- Todos have:
  - Unique ID (UUID)
  - Title & optional description
  - Priority: High | Medium | Low
  - Status: Pending | In Progress | Done
  - Creation timestamp
  - Optional due date (UTC) and tags
  - Optional recurrence: Daily | Weekly | Custom("...")
  - Optional parent/subtasks relationships

---

## 🚀 Installation

Clone the repository and install the binary locally:

```bash
git clone https://github.com/leocm889/todo_list.git
cd todo_list
cargo install --path .
```

This will install the binary to `~/.cargo/bin/todo`.
Make sure `~/.cargo/bin` is in your `$PATH`.

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
# Flags for add:
# -t/--title (required), -d/--description, -p/--priority, -s/--status,
# -D/--due-date (YYYY-MM-DD), -r/--recurrence, -g/--tags, -P/--parent-id, -u/--subtasks

todo add -t "Buy milk" -d "From the store" -p high -s pending -D 2025-09-20 -g groceries,errands
```

**View todos**

```bash
# --sort-by created|priority|status|due-date|overdue (default: created)
todo list --sort-by due-date
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

**Search by due date**

```bash
todo search --due-date 2025-09-20
```

**Search by tags (comma separated)**

```bash
todo search --tags work,urgent
```

**Update a todo**

```bash
# Update uses flags like add, but all are optional except --id
todo update --id <UUID> -t "Buy bread" -s in_progress -D 2025-09-21
```

**Delete a todo**

```bash
todo delete --id <UUID>
```

**Send desktop notifications**

```bash
# Shows notifications for tasks due now or overdue
todo notify
```

Notes:
- On Linux, ensure a notification daemon is running (e.g., GNOME/KDE notifier, dunst).

---

## 📂 Project Structure

```bash
src/
├── main.rs       # CLI entry point (clap) and command dispatch, handles --menu
├── lib.rs        # Library exports and helpers used by tests
├── cli.rs        # Clap command definitions
├── input.rs      # DTOs for CLI to core (Add/Update/Search input structs)
├── menu.rs       # Interactive menu UI (stdin-driven)
├── todo.rs       # Todo model + Display + interactive helpers
├── todo_cli.rs   # Non-interactive CLI handlers (add/list/search/update/delete)
├── storage.rs    # JSON persistence (load/save)
├── notify.rs     # Desktop notifications for due/overdue tasks
├── priority.rs   # Priority enum + Display
├── status.rs     # Status enum + Display
├── recurrence.rs # Recurrence enum + Display/FromStr (Daily/Weekly/Custom)
├── utils.rs      # Input helpers
└── tests/        # Integration tests
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
    "created_at": "2025-09-04T12:34:56",
    "due_date": "2025-09-20T00:00:00Z",
    "tags": ["groceries", "errands"],
    "recurrence": null,
    "parent_id": null,
    "subtasks": []
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

- Background notifier/daemon mode
- Support for multiple storage backends (SQLite, PostgreSQL)
- Export todos as CSV/Markdown
- More advanced search/filtering
- Colored output for better readability

---

## ✍️ Author

- leocm889

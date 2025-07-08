# Todo CLI

A fast, colorful, and feature-rich personal task management CLI tool written in Rust.

![Demo](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-yellow.svg)

## ✨ Features

- 🎨 **Colorful output** with priority-based colors and visual indicators
- 📅 **Flexible date parsing** (ISO dates, natural language, relative dates)
- 🏷️ **Tag system** for organizing tasks
- ⚡ **Fast performance** (< 100ms command execution)
- 💾 **Smart data storage** (XDG config, environment variables, custom paths)
- 🔍 **Powerful filtering** (by tags, completion status)
- 📊 **Priority system** (1-5, with 1 being highest priority)
- ⚠️ **Overdue detection** with visual warnings
- 🔄 **Task editing** and management
- ✅ **Completion tracking** with timestamps

## 🚀 Installation

### From Source

```bash
git clone https://github.com/kcurtet/todo-cli.git
cd todo-cli
cargo build --release
```

The binary will be available at `target/release/todo-cli`. You can add it to your PATH or create an alias:

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
alias todo="~/path/to/todo-cli/target/release/todo-cli"
```

### Using Nix

If you have Nix with flakes enabled:

```bash
# Install directly from the repository
nix profile install github:kcurtet/todo-cli

# Or run without installing
nix run github:kcurtet/todo-cli
```

### Shell Completions

Shell completions are automatically installed when using Nix. For manual installation:

```bash
# Generate completions using the built binary
./target/release/todo-cli completions bash > ~/.local/share/bash-completion/completions/todo
./target/release/todo-cli completions zsh > ~/.zsh/completions/_todo
./target/release/todo-cli completions fish > ~/.config/fish/completions/todo.fish

# Or use the included installation script
./scripts/install-completions.sh

# For all shells
./scripts/install-completions.sh --all

# For specific shell
./scripts/install-completions.sh --shell zsh
```

#### Shell-specific Setup

**Bash**: Completions should work automatically if you have bash-completion installed.

**Zsh**: Add to your `~/.zshrc`:
```bash
fpath=(~/.zsh/completions $fpath)
autoload -U compinit && compinit
```

**Fish**: Completions are automatically loaded from `~/.config/fish/completions/`.

## 📖 Usage

### Adding Tasks

```bash
# Simple task
todo add "Buy groceries"

# Task with priority (1-5, 1 = highest)
todo add "Fix critical bug" -p 1

# Task with due date
todo add "Submit report" -d "2025-07-15"
todo add "Call client" -d tomorrow
todo add "Team meeting" -d "next friday"

# Task with tags
todo add "Learn Rust" -t learning -t programming

# Complex task with all options
todo add "Complete project proposal" -p 2 -d "next monday" -t work -t urgent
```

### Listing Tasks

```bash
# List all incomplete tasks
todo list

# List all tasks (including completed)
todo list -c

# Filter by tag
todo list -t work
todo list -t personal

# Exclude specific tags
todo list --exclude-tag meetings

# Combine filters
todo list -t work --exclude-tag low-priority -c
```

### Managing Tasks

```bash
# Mark task as complete
todo complete 1

# Edit a task
todo edit 1 -d "2025-07-20"           # Change due date
todo edit 1 -p 3                      # Change priority
todo edit 1 -d "Fix login issue"      # Change description
todo edit 1 -t urgent                 # Add tags (keeps existing ones)

# Delete a task
todo delete 1
```

### Data File Options

```bash
# Use custom data file
todo --data-file /path/to/my-tasks.json list

# Use environment variable
export TODO_DATA_FILE=/path/to/my-tasks.json
todo list

# Default locations (in order of preference):
# 1. $TODO_DATA_FILE environment variable
# 2. --data-file command line flag  
# 3. ~/.config/todo/tasks.json (XDG config)
# 4. ~/.todo.json (fallback)
```

## 🎨 Visual Features

### Priority Colors
- **P1** (Highest): 🔴 Red
- **P2** (High): 🟡 Yellow  
- **P3** (Medium): 🔵 Blue
- **P4** (Low): 🟢 Green
- **P5** (Lowest): 🔵 Cyan

### Status Indicators
- **Overdue tasks**: ⚠️ Red warning with bold text
- **Completed tasks**: ~~Strikethrough~~ and dimmed
- **Tags**: #tag in bright cyan
- **Due dates**: Color-coded by urgency
  - Today: Yellow bold
  - 1-3 days: Yellow
  - Overdue: Red bold

### Date Formats Supported

```bash
# ISO dates
todo add "Task" -d "2025-07-15"
todo add "Task" -d "2025/07/15"

# Relative dates
todo add "Task" -d today
todo add "Task" -d tomorrow

# Weekdays
todo add "Task" -d monday
todo add "Task" -d friday

# Natural language (via chrono-english)
todo add "Task" -d "next friday"
todo add "Task" -d "in 3 days"
```

## 📊 Example Output

```
[1] P1 Fix critical security vulnerability #security #urgent ⚠️ (due yesterday)
[2] P2 Review pull requests #work (due today)
[3] P3 Update documentation #docs (due tomorrow)
[4] P4 Learn new framework #learning #personal (due in 5 days)
[5] ~~Completed task~~ #work (completed today)

ℹ Showing 5 tasks. Total: 10, Completed: 3, Overdue: 1
```

## 🗂️ Data Storage

Tasks are stored in JSON format with the following structure:

```json
{
  "tasks": [
    {
      "id": 1,
      "description": "Buy groceries",
      "priority": 2,
      "due_date": "2025-07-09T23:59:59+02:00",
      "tags": ["shopping"],
      "completed": false,
      "created_at": "2025-07-08T13:29:43.254043558+02:00",
      "completed_at": null
    }
  ],
  "next_id": 2
}
```

### Data Location Priority

1. `$TODO_DATA_FILE` environment variable
2. `--data-file` command line flag
3. `~/.config/todo/tasks.json` (XDG Base Directory)
4. `~/.todo.json` (fallback)
5. `./tasks.json` (last resort)

## 🔧 Command Reference

### Global Options
- `--data-file <PATH>` - Use custom data file location

### Commands

#### `add`
Add a new task.

**Options:**
- `-p, --priority <1-5>` - Set priority (1 = highest, 5 = lowest)
- `-d, --due <DATE>` - Set due date
- `-t, --tags <TAG>` - Add tags (can be used multiple times)

#### `list`
List tasks with optional filtering.

**Options:**
- `-t, --tag <TAG>` - Show only tasks with this tag
- `--exclude-tag <TAG>` - Hide tasks with this tag
- `-c, --completed` - Include completed tasks

#### `complete`
Mark a task as completed.

**Arguments:**
- `<ID>` - Task ID to complete

#### `edit`
Edit an existing task.

**Arguments:**
- `<ID>` - Task ID to edit

**Options:**
- `-d, --description <TEXT>` - Update description
- `-p, --priority <1-5>` - Update priority
- `--due <DATE>` - Update due date
- `-t, --tags <TAG>` - Add tags (existing tags are preserved)

#### `delete`
Delete a task permanently.

**Arguments:**
- `<ID>` - Task ID to delete

#### `completions`
Generate shell completions.

**Arguments:**
- `<SHELL>` - Shell to generate completions for (bash, zsh, fish, elvish, powershell)

**Usage:**
```bash
# Generate and save completions
todo completions bash > ~/.local/share/bash-completion/completions/todo
todo completions zsh > ~/.zsh/completions/_todo
todo completions fish > ~/.config/fish/completions/todo.fish
```

## 🛠️ Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Build with completions generation
./scripts/build.sh

# Run tests
cargo test

# Using Nix
nix build
```

### Dependencies

- **clap** - Command line argument parsing
- **serde** - Serialization/deserialization
- **chrono** - Date and time handling
- **colored** - Terminal colors
- **thiserror** - Error handling
- **chrono-english** - Natural language date parsing
- **dirs** - System directory locations

### Performance

- **Startup time**: < 100ms (< 50ms in release mode)
- **Memory usage**: < 10MB for 1000+ tasks
- **Command execution**: < 100ms for typical operations

## 🐛 Error Handling

The CLI provides helpful error messages for common issues:

```bash
# Invalid priority
$ todo add "Task" -p 10
✗ Invalid priority value: 10. Priority must be between 1 and 5

# Task not found
$ todo complete 999
✗ Task not found with ID: 999

# Invalid date format
$ todo add "Task" -d "invalid-date"
✗ Unable to parse date: 'invalid-date'. Try formats like: YYYY-MM-DD, today, tomorrow, monday, etc.
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 👨‍💻 Author

**Kevin Curtet**
- Email: kcurtet@gmail.com
- GitHub: [@kcurtet](https://github.com/kcurtet)

## 🎯 Roadmap

- [ ] Recurring tasks
- [ ] Task dependencies
- [ ] Export/import functionality
- [ ] Calendar integration
- [ ] Notifications
- [ ] Sub-tasks
- [ ] Time tracking

---

Made with ❤️ in Rust

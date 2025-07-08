# Todo CLI - Development Brief for CodePilot

## Project Overview
Build a personal task management CLI tool in Rust with colorful output, priorities, due dates, and tags.

## Core Requirements

### Data Model
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub priority: Option<u8>, // 1-5, 1 = highest
    pub due_date: Option<DateTime<Local>>,
    pub tags: Vec<String>,
    pub completed: bool,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
}
```

### Commands Structure
```rust
#[derive(Subcommand)]
pub enum Commands {
    Add {
        description: String,
        #[arg(short, long)] priority: Option<u8>,
        #[arg(short, long)] due: Option<String>,
        #[arg(short, long)] tags: Vec<String>,
    },
    List {
        #[arg(short, long)] tag: Option<String>,
        #[arg(long)] exclude_tag: Option<String>,
        #[arg(short, long)] completed: bool,
    },
    Complete { id: u64 },
    Edit {
        id: u64,
        #[arg(short, long)] description: Option<String>,
        #[arg(short, long)] priority: Option<u8>,
        #[arg(short, long)] due: Option<String>,
    },
    Delete { id: u64 },
}
```

## Key Features

### Storage
- **Location**: `~/.config/todo/tasks.json` (XDG), fallback to `~/.todo.json`
- **Override**: `--data-file` flag or `TODO_DATA_FILE` env var
- **Format**: JSON with auto-save

### Date Parsing
Support multiple formats:
- ISO dates: `2025-07-15`
- Relative: `today`, `tomorrow`, `next friday`
- Natural language via `chrono-english`

### Sorting & Filtering
- **Default sort**: Due date → Priority → Creation time
- **Priority**: 1 (highest) to 5 (lowest)
- **Tag filtering**: Include/exclude specific tags
- **Overdue detection**: Highlight overdue tasks

### Color Coding
- **Priority colors**: Red (1), Yellow (2), Blue (3), Green (4), Cyan (5)
- **Overdue tasks**: Red with warning symbol
- **Completed tasks**: Strikethrough and dimmed
- **Tags**: Bright cyan with # prefix

## Required Dependencies
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
colored = "2.0"
thiserror = "1.0"
chrono-english = "0.1"
dirs = "5.0"
```

## Key Implementation Notes

### Error Handling
- Use `thiserror` for structured errors
- Provide user-friendly messages with examples
- Handle data corruption gracefully

### Performance Requirements
- Startup time: < 50ms
- Command execution: < 100ms for 1000+ tasks
- Memory usage: < 10MB

### File Structure
```
src/
├── main.rs          # Entry point
├── cli.rs           # Command parsing
├── task.rs          # Task model
├── storage.rs       # Data persistence
├── date_parser.rs   # Date parsing
├── filter.rs        # Task filtering
├── renderer.rs      # Color output
└── error.rs         # Error types
scripts/
├── build.sh         # Build script with completion generation
└── install-completions.sh  # Shell completion installer
```

## Development Priorities
1. **Core data model** and JSON storage
2. **CLI parsing** with clap
3. **Date parsing** with multiple formats
4. **Sorting and filtering** logic
5. **Colorful output** rendering
6. **Error handling** and UX polish

## Example Usage
```bash
# Add tasks
todo add "Buy groceries" -p 2 -d tomorrow -t shopping
todo add "Fix bug" -p 1 -d "2025-07-15" -t work

# List tasks
todo list              # All tasks
todo list -t work      # Only work tasks
todo list --exclude-tag personal

# Complete and edit
todo complete 1
todo edit 2 -p 3 -d "next friday"
todo delete 3
```

## Success Criteria
- Fast, responsive CLI interface
- Intuitive command structure
- Rich, colorful output
- Reliable data persistence
- Comprehensive error handling
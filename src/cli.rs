use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A fast, colorful, and feature-rich personal task management CLI tool")]
#[command(version = "0.1.0")]
#[command(author = "Kevin Curtet <kcurtet@gmail.com>")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the data file
    #[arg(long, global = true)]
    pub data_file: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Task description
        description: String,

        /// Priority (1-5, 1 = highest)
        #[arg(short, long)]
        priority: Option<u8>,

        /// Due date (YYYY-MM-DD, today, tomorrow, etc.)
        #[arg(short, long)]
        due: Option<String>,

        /// Tags for the task
        #[arg(short, long)]
        tags: Vec<String>,
    },

    /// List tasks
    List {
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Exclude tasks with this tag
        #[arg(long)]
        exclude_tag: Option<String>,

        /// Show completed tasks
        #[arg(short, long)]
        completed: bool,
    },

    /// Mark a task as complete
    Complete {
        /// Task ID to complete
        id: u64,
    },

    /// Edit an existing task
    Edit {
        /// Task ID to edit
        id: u64,

        /// New description
        #[arg(short, long)]
        description: Option<String>,

        /// New priority (1-5, 1 = highest)
        #[arg(short, long)]
        priority: Option<u8>,

        /// New due date
        #[arg(long)]
        due: Option<String>,

        /// Add tags (existing tags will be kept)
        #[arg(short, long)]
        tags: Vec<String>,
    },

    /// Delete a task
    Delete {
        /// Task ID to delete
        id: u64,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

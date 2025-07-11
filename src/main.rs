mod cli;
mod date_parser;
mod error;
mod filter;
mod renderer;
mod storage;
mod task;

use chrono::{DateTime, Local};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::io;
use std::process;

use cli::{Cli, Commands};
use date_parser::{parse_date, parse_date_from_words};
use error::{Result, TodoError};
use filter::sort_tasks;
use renderer::{render_error, render_info, render_success, render_task_list};
use storage::{TaskStorage, get_data_file_path};
use task::Task;

/// Entry point for the todo CLI application.
fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        render_error(&e.to_string());
        process::exit(1);
    }
}

/// Main application logic for handling commands and errors.
fn run(cli: Cli) -> Result<()> {
    let data_path = get_data_file_path(cli.data_file.as_deref());
    let mut storage = TaskStorage::load_from_file(&data_path)?;

    match cli.command {
        Commands::Add {
            description,
            priority,
            due,
            tags,
        } => {
            // Parse tags from description words starting with '@' and parse date-like phrase
            let mut desc_words = Vec::new();
            let mut parsed_tags = tags;
            let mut parsed_due = due;
            for word in &description {
                if word.starts_with('@') && word.len() > 1 {
                    let tag = word[1..].to_string();
                    if !parsed_tags.contains(&tag) {
                        parsed_tags.push(tag);
                    }
                } else {
                    desc_words.push(word.clone());
                }
            }
            // Try to extract a date phrase from the remaining words if no due date was given
            if parsed_due.is_none() {
                let word_refs: Vec<&str> = desc_words.iter().map(|s| s.as_str()).collect();
                if let Some(dt) = parse_date_from_words(&word_refs) {
                    parsed_due = Some(dt.to_rfc3339());
                    // Remove the date phrase from the description
                    // (optional: not implemented here for simplicity)
                }
            }
            let description = desc_words.join(" ").trim().to_string();
            add_task(&mut storage, description, priority, parsed_due, parsed_tags)?;
            storage.save_to_file(&data_path)?;
            render_success("Task added successfully");
        }

        Commands::List {
            tag,
            exclude_tag,
            completed,
        } => {
            list_tasks(&storage, tag.as_deref(), exclude_tag.as_deref(), completed);
        }

        Commands::Complete { id } => {
            complete_task(&mut storage, id)?;
            storage.save_to_file(&data_path)?;
            render_success(&format!("Task {} marked as complete", id));
        }

        Commands::Edit {
            id,
            description,
            priority,
            due,
            tags,
        } => {
            edit_task(&mut storage, id, description, priority, due, tags)?;
            storage.save_to_file(&data_path)?;
            render_success(&format!("Task {} updated successfully", id));
        }

        Commands::Delete { id } => {
            storage.delete_task(id)?;
            storage.save_to_file(&data_path)?;
            render_success(&format!("Task {} deleted successfully", id));
        }

        Commands::Completions { shell } => {
            generate_completions(shell);
            return Ok(());
        }
    }

    Ok(())
}

/// Adds a new task to the storage.
fn add_task(
    storage: &mut TaskStorage,
    description: String,
    priority: Option<u8>,
    due: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    // Validate priority
    if let Some(p) = priority {
        if !(1..=5).contains(&p) {
            return Err(TodoError::InvalidPriority(p));
        }
    }

    // Validate tags
    for tag in &tags {
        if tag.trim().is_empty() {
            return Err(TodoError::InvalidTag(tag.clone()));
        }
    }

    let mut task = Task::new(0, description); // ID will be set by storage
    task.priority = priority;
    task.tags = tags.into_iter().map(|t| t.trim().to_string()).collect();

    // Parse due date if provided
    if let Some(due_str) = due {
        // Try RFC3339 first (for parse_date_from_words result)
        if let Ok(dt) = DateTime::parse_from_rfc3339(&due_str) {
            task.due_date = Some(dt.with_timezone(&Local));
        } else {
            task.due_date = Some(parse_date(&due_str)?);
        }
    }

    let task_id = storage.add_task(task);
    render_info(&format!("Created task with ID: {}", task_id));

    Ok(())
}

/// Lists tasks based on the provided filters.
fn list_tasks(
    storage: &TaskStorage,
    include_tag: Option<&str>,
    exclude_tag: Option<&str>,
    show_completed: bool,
) {
    let mut tasks = storage.get_filtered_tasks(include_tag, exclude_tag, show_completed);

    if tasks.is_empty() {
        render_info("No tasks found matching the criteria");
        return;
    }

    sort_tasks(&mut tasks);
    render_task_list(&tasks);

    // Show summary
    let total_tasks = storage.tasks.len();
    let completed_tasks = storage.tasks.iter().filter(|t| t.completed).count();
    let overdue_tasks = storage.tasks.iter().filter(|t| t.is_overdue()).count();

    println!();
    render_info(&format!(
        "Showing {} tasks. Total: {}, Completed: {}, Overdue: {}",
        tasks.len(),
        total_tasks,
        completed_tasks,
        overdue_tasks
    ));
}

/// Marks a task as complete.
fn complete_task(storage: &mut TaskStorage, id: u64) -> Result<()> {
    let task = storage
        .get_task_mut(id)
        .ok_or(TodoError::TaskNotFound(id))?;

    if task.completed {
        render_info(&format!("Task {} is already completed", id));
        return Ok(());
    }

    task.complete();
    Ok(())
}

/// Edits an existing task in the storage.
fn edit_task(
    storage: &mut TaskStorage,
    id: u64,
    description: Option<String>,
    priority: Option<u8>,
    due: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    // Validate priority
    if let Some(p) = priority {
        if !(1..=5).contains(&p) {
            return Err(TodoError::InvalidPriority(p));
        }
    }

    // Validate tags
    for tag in &tags {
        if tag.trim().is_empty() {
            return Err(TodoError::InvalidTag(tag.clone()));
        }
    }

    let task = storage
        .get_task_mut(id)
        .ok_or(TodoError::TaskNotFound(id))?;

    // Update description
    if let Some(desc) = description {
        task.description = desc;
    }

    // Update priority
    if let Some(p) = priority {
        task.priority = Some(p);
    }

    // Update due date
    if let Some(due_str) = due {
        task.due_date = Some(parse_date(&due_str)?);
    }

    // Add new tags (keep existing ones)
    if !tags.is_empty() {
        for tag in tags {
            let tag = tag.trim().to_string();
            if !task.tags.contains(&tag) {
                task.tags.push(tag);
            }
        }
    }

    Ok(())
}

/// Generates shell completions for the CLI.
fn generate_completions(shell: clap_complete::Shell) {
    let mut cmd = Cli::command();
    let bin_name = "todo";

    generate(shell, &mut cmd, bin_name, &mut io::stdout());
}

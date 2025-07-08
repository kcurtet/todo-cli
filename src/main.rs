mod cli;
mod date_parser;
mod error;
mod filter;
mod renderer;
mod storage;
mod task;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::io;
use std::process;

use cli::{Cli, Commands};
use date_parser::parse_date;
use error::{Result, TodoError};
use filter::sort_tasks;
use renderer::{render_error, render_info, render_success, render_task_list};
use storage::{TaskStorage, get_data_file_path};
use task::Task;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        render_error(&e.to_string());
        process::exit(1);
    }
}

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
            add_task(&mut storage, description, priority, due, tags)?;
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
        task.due_date = Some(parse_date(&due_str)?);
    }

    let task_id = storage.add_task(task);
    render_info(&format!("Created task with ID: {}", task_id));

    Ok(())
}

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

fn generate_completions(shell: clap_complete::Shell) {
    let mut cmd = Cli::command();
    let bin_name = "todo";

    generate(shell, &mut cmd, bin_name, &mut io::stdout());
}

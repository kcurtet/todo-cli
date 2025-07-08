use crate::error::{Result, TodoError};
use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Persistent storage for tasks and their IDs.
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStorage {
    /// All tasks in the storage.
    pub tasks: Vec<Task>,
    /// Next available task ID.
    pub next_id: u64,
}

impl Default for TaskStorage {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
}

impl TaskStorage {
    /// Load tasks from a JSON file at the given path.
    ///
    /// Returns a default storage if the file does not exist or is empty.
    pub fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)?;
        if content.trim().is_empty() {
            return Ok(Self::default());
        }

        match serde_json::from_str(&content) {
            Ok(storage) => Ok(storage),
            Err(e) => {
                let backup_path = path.with_extension("json.backup");
                if let Err(backup_err) = fs::copy(path, &backup_path) {
                    eprintln!("Warning: Failed to create backup: {}", backup_err);
                }
                Err(TodoError::DataCorruption(format!(
                    "Failed to parse data file. Backup created at {:?}. Error: {}",
                    backup_path, e
                )))
            }
        }
    }

    /// Save tasks to a JSON file at the given path.
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Add a new task to the storage and return its ID.
    pub fn add_task(&mut self, mut task: Task) -> u64 {
        task.id = self.next_id;
        let task_id = self.next_id;
        self.next_id += 1;
        self.tasks.push(task);
        task_id
    }

    #[allow(dead_code)]
    /// Get a reference to a task by its ID.
    pub fn get_task(&self, id: u64) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    /// Get a mutable reference to a task by its ID.
    pub fn get_task_mut(&mut self, id: u64) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    /// Delete a task by its ID.
    pub fn delete_task(&mut self, id: u64) -> Result<()> {
        let index = self
            .tasks
            .iter()
            .position(|task| task.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;
        self.tasks.remove(index);
        Ok(())
    }

    /// Get tasks filtered by included/excluded tags and completion status.
    pub fn get_filtered_tasks(
        &self,
        include_tag: Option<&str>,
        exclude_tag: Option<&str>,
        show_completed: bool,
    ) -> Vec<&Task> {
        crate::filter::filter_tasks(&self.tasks, include_tag, exclude_tag, show_completed)
    }
}

/// Get the data file path, prioritizing environment variable, then custom path, then default locations.
pub fn get_data_file_path(custom_path: Option<&str>) -> PathBuf {
    if let Ok(env_path) = std::env::var("TODO_DATA_FILE") {
        return PathBuf::from(env_path);
    }

    if let Some(path) = custom_path {
        return PathBuf::from(path);
    }

    if let Some(config_dir) = dirs::config_dir() {
        let xdg_path = config_dir.join("todo").join("tasks.json");
        return xdg_path;
    }

    if let Some(home_dir) = dirs::home_dir() {
        return home_dir.join(".todo.json");
    }

    PathBuf::from("tasks.json")
}

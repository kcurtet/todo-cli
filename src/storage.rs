use crate::error::{Result, TodoError};
use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStorage {
    pub tasks: Vec<Task>,
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
                // If we can't parse the JSON, try to recover by creating a backup
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

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn add_task(&mut self, mut task: Task) -> u64 {
        task.id = self.next_id;
        let task_id = self.next_id;
        self.next_id += 1;
        self.tasks.push(task);
        task_id
    }

    #[allow(dead_code)]
    pub fn get_task(&self, id: u64) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn get_task_mut(&mut self, id: u64) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    pub fn delete_task(&mut self, id: u64) -> Result<()> {
        let index = self
            .tasks
            .iter()
            .position(|task| task.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;
        self.tasks.remove(index);
        Ok(())
    }

    pub fn get_filtered_tasks(
        &self,
        include_tag: Option<&str>,
        exclude_tag: Option<&str>,
        show_completed: bool,
    ) -> Vec<&Task> {
        crate::filter::filter_tasks(&self.tasks, include_tag, exclude_tag, show_completed)
    }
}

pub fn get_data_file_path(custom_path: Option<&str>) -> PathBuf {
    // Check environment variable first
    if let Ok(env_path) = std::env::var("TODO_DATA_FILE") {
        return PathBuf::from(env_path);
    }

    // Use custom path if provided
    if let Some(path) = custom_path {
        return PathBuf::from(path);
    }

    // Try XDG config directory
    if let Some(config_dir) = dirs::config_dir() {
        let xdg_path = config_dir.join("todo").join("tasks.json");
        return xdg_path;
    }

    // Fallback to home directory
    if let Some(home_dir) = dirs::home_dir() {
        return home_dir.join(".todo.json");
    }

    // Last resort: current directory
    PathBuf::from("tasks.json")
}

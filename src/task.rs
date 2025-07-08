use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

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

impl Task {
    pub fn new(id: u64, description: String) -> Self {
        Self {
            id,
            description,
            priority: None,
            due_date: None,
            tags: Vec::new(),
            completed: false,
            created_at: Local::now(),
            completed_at: None,
        }
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            !self.completed && due_date < Local::now()
        } else {
            false
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now());
    }

    pub fn matches_tag_filter(&self, include_tag: Option<&str>, exclude_tag: Option<&str>) -> bool {
        if let Some(tag) = include_tag {
            if !self.tags.iter().any(|t| t == tag) {
                return false;
            }
        }

        if let Some(tag) = exclude_tag {
            if self.tags.iter().any(|t| t == tag) {
                return false;
            }
        }

        true
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        // First by completion status (incomplete tasks first)
        match (self.completed, other.completed) {
            (false, true) => return Ordering::Less,
            (true, false) => return Ordering::Greater,
            _ => {}
        }

        // Then by due date (closest first)
        match (&self.due_date, &other.due_date) {
            (Some(a), Some(b)) => {
                let ord = a.cmp(b);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            (Some(_), None) => return Ordering::Less,
            (None, Some(_)) => return Ordering::Greater,
            _ => {}
        }

        // Then by priority (1 is highest priority)
        match (&self.priority, &other.priority) {
            (Some(a), Some(b)) => {
                let ord = a.cmp(b);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            (Some(_), None) => return Ordering::Less,
            (None, Some(_)) => return Ordering::Greater,
            _ => {}
        }

        // Finally by creation time (oldest first)
        self.created_at.cmp(&other.created_at)
    }
}

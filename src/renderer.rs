use crate::task::Task;
use chrono::{DateTime, Local};
use colored::*;

pub fn render_task_list(tasks: &[&Task]) {
    if tasks.is_empty() {
        println!("{}", "No tasks found.".dimmed());
        return;
    }

    for task in tasks {
        render_task(task);
    }
}

pub fn render_task(task: &Task) {
    let mut output = String::new();

    // Task ID
    output.push_str(&format!("[{}] ", task.id.to_string().cyan().bold()));

    // Priority indicator
    if let Some(priority) = task.priority {
        let priority_str = format!("P{} ", priority);
        let colored_priority = match priority {
            1 => priority_str.red().bold(),
            2 => priority_str.yellow().bold(),
            3 => priority_str.blue().bold(),
            4 => priority_str.green().bold(),
            5 => priority_str.cyan().bold(),
            _ => priority_str.white().bold(),
        };
        output.push_str(&colored_priority.to_string());
    }

    // Description
    let description = if task.completed {
        task.description.strikethrough().dimmed().to_string()
    } else if task.is_overdue() {
        format!("⚠️  {}", task.description.red().bold())
    } else {
        task.description.clone()
    };
    output.push_str(&description);

    // Tags
    if !task.tags.is_empty() {
        output.push(' ');
        for tag in &task.tags {
            output.push_str(&format!("#{} ", tag.bright_cyan()));
        }
    }

    // Due date
    if let Some(due_date) = task.due_date {
        let due_str = format_due_date(due_date, task.is_overdue());
        output.push_str(&format!(" {}", due_str));
    }

    // Completion status
    if task.completed {
        if let Some(completed_at) = task.completed_at {
            output.push_str(&format!(
                " {}",
                format!("(completed {})", format_relative_date(completed_at))
                    .green()
                    .dimmed()
            ));
        } else {
            output.push_str(&format!(" {}", "(completed)".green().dimmed()));
        }
    }

    println!("{}", output);
}

fn format_due_date(due_date: DateTime<Local>, is_overdue: bool) -> ColoredString {
    let due_str = format!("(due {})", format_relative_date(due_date));

    if is_overdue {
        due_str.red().bold()
    } else {
        let now = Local::now();
        let days_until = (due_date.date_naive() - now.date_naive()).num_days();

        match days_until {
            0 => due_str.yellow().bold(),
            1..=3 => due_str.yellow(),
            _ => due_str.normal(),
        }
    }
}

fn format_relative_date(date: DateTime<Local>) -> String {
    let now = Local::now();
    let date_naive = date.date_naive();
    let now_naive = now.date_naive();

    let days_diff = (date_naive - now_naive).num_days();

    match days_diff {
        0 => "today".to_string(),
        1 => "tomorrow".to_string(),
        -1 => "yesterday".to_string(),
        2..=7 => format!("in {} days", days_diff),
        -7..=-2 => format!("{} days ago", -days_diff),
        _ => date.format("%Y-%m-%d").to_string(),
    }
}

pub fn render_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn render_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message.red());
}

#[allow(dead_code)]
pub fn render_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message.yellow());
}

pub fn render_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_format_relative_date() {
        let now = Local::now();
        let today = now.date_naive();

        let tomorrow = Local
            .from_local_datetime(&today.succ_opt().unwrap().and_hms_opt(12, 0, 0).unwrap())
            .unwrap();
        assert_eq!(format_relative_date(tomorrow), "tomorrow");

        let yesterday = Local
            .from_local_datetime(&today.pred_opt().unwrap().and_hms_opt(12, 0, 0).unwrap())
            .unwrap();
        assert_eq!(format_relative_date(yesterday), "yesterday");
    }
}

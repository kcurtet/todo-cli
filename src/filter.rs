use crate::task::Task;

pub fn sort_tasks(tasks: &mut [&Task]) {
    tasks.sort();
}

pub fn filter_tasks<'a>(
    tasks: &'a [Task],
    include_tag: Option<&str>,
    exclude_tag: Option<&str>,
    show_completed: bool,
) -> Vec<&'a Task> {
    tasks
        .iter()
        .filter(|task| {
            // Filter by completion status
            if !show_completed && task.completed {
                return false;
            }

            // Filter by tags
            task.matches_tag_filter(include_tag, exclude_tag)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_task(id: u64, description: &str, tags: Vec<String>, completed: bool) -> Task {
        let mut task = Task::new(id, description.to_string());
        task.tags = tags;
        task.completed = completed;
        task
    }

    #[test]
    fn test_filter_by_tag() {
        let tasks = vec![
            create_test_task(1, "Task 1", vec!["work".to_string()], false),
            create_test_task(2, "Task 2", vec!["personal".to_string()], false),
            create_test_task(
                3,
                "Task 3",
                vec!["work".to_string(), "urgent".to_string()],
                false,
            ),
        ];

        let filtered = filter_tasks(&tasks, Some("work"), None, false);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].id, 1);
        assert_eq!(filtered[1].id, 3);
    }

    #[test]
    fn test_exclude_tag() {
        let tasks = vec![
            create_test_task(1, "Task 1", vec!["work".to_string()], false),
            create_test_task(2, "Task 2", vec!["personal".to_string()], false),
            create_test_task(
                3,
                "Task 3",
                vec!["work".to_string(), "urgent".to_string()],
                false,
            ),
        ];

        let filtered = filter_tasks(&tasks, None, Some("urgent"), false);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].id, 1);
        assert_eq!(filtered[1].id, 2);
    }

    #[test]
    fn test_filter_completed() {
        let tasks = vec![
            create_test_task(1, "Task 1", vec![], false),
            create_test_task(2, "Task 2", vec![], true),
            create_test_task(3, "Task 3", vec![], false),
        ];

        let filtered = filter_tasks(&tasks, None, None, false);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].id, 1);
        assert_eq!(filtered[1].id, 3);

        let filtered_with_completed = filter_tasks(&tasks, None, None, true);
        assert_eq!(filtered_with_completed.len(), 3);
    }
}

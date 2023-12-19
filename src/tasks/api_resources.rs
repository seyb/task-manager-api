use serde::Serialize;

use crate::tasks::domain::Task;

#[derive(Serialize)]
pub struct TaskApiResource {
    title: String,
    description: String,
    completed_at: Option<String>
}

impl From<Task> for TaskApiResource {
    fn from(t: Task) -> Self {
        let completed = t.completed_at.map(|date| "this is a date".to_string());
        Self {
            title: t.title,
            description: t.description,
            completed_at: completed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_tasks() {
        let expected_title = "tested title";
        let task = Task::new(expected_title);
        let api_resource = TaskApiResource::from(task);
        assert_eq!(expected_title, api_resource.title)
    }


    #[test]
    fn it_maps_title() {
        let task = Task::new("test title");
        let api_resource = TaskApiResource::from(task);
        assert_eq!("test title", api_resource.title);
    }


    #[test]
    fn it_does_not_map_completed_at_if_not_present() {
        let task = Task::new("test title");
        let api_resource = TaskApiResource::from(task);
        assert!(api_resource.completed_at.is_none());
    }
}

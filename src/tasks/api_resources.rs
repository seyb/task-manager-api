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
    use crate::tasks::api_resources::TaskApiResource;
    use crate::tasks::domain::Task;

    #[test]
    fn it_serializes_tasks() {
        let expected_title = "tested title";
        let task = Task::new(expected_title);
        let api_resource = TaskApiResource::from(task);
        assert_eq!(expected_title, api_resource.title)
    }

}


mod domain;

use serde::Serialize;
use std::time::SystemTime;
use axum::http::StatusCode;
use axum::Json;

#[derive(Serialize)]
pub struct TasksSerializer {
    id: String,
    description: String,
    complete: Option<SystemTime>
}

trait ApiResourceMapper {
    type DomainObject;
    fn map(obj: DomainObject) -> T;
}


impl ApiResourceMapper for Task {
    type DomainObject = TasksSerializer;
}
pub async fn get_tasks() -> (StatusCode, Json<Vec<TasksSerializer>>) {
    let collections = vec![TasksSerializer { id: "test".to_string(), description:"description".to_string(), complete: None }];
    (StatusCode::OK, Json(collections))
}


#[cfg(test)]
mod tests {
    use crate::tasks::domain::Task;

    #[test]
    fn it_serializes_tasks() {
        let task = Task::new("test");

    }

}

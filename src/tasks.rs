use axum::http::StatusCode;
use axum::Json;


use api_resources::TaskApiResource;
use crate::tasks::domain::Task;

mod domain;
mod api_resources;

pub async fn get_tasks() -> (StatusCode, Json<Vec<TaskApiResource>>) {
    let task1 = Task::new("task one");
    let mut task2 = Task::new("task two");
    task2.complete();
    let collections = vec![TaskApiResource::from(task1), TaskApiResource::from(task2)];
    (StatusCode::OK, Json(collections))
}

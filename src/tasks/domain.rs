use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub completed_at: Option<SystemTime>,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            description: "".to_string(),
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        match self.completed_at {
            None => self.completed_at = Some(SystemTime::now()),
            x => self.completed_at = x,
        }
    }
    pub fn uncomplete(&mut self) {
        self.completed_at = None
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::domain::Task;

    #[test]
    fn it_inits_task() {
        let expected_title = "new Task";
        let task = Task::new(expected_title);
        assert_eq!(task.title, expected_title.to_string());
        assert_eq!(task.description, "".to_string());
        assert_eq!(task.completed_at, None)
    }

    #[test]
    fn it_completes_a_task() {
        let expected_title = "new Task";
        let mut task = Task::new(expected_title);
        task.complete();
        assert_ne!(task.completed_at, None)
    }

    #[test]
    fn it_uncompletes_a_task() {
        let expected_title = "new Task";
        let mut task = Task::new(expected_title);
        task.complete();
        task.uncomplete();
        assert_eq!(task.completed_at, None)
    }

    #[test]
    fn it_does_not_set_completes_mulitple_times() {
        let expected_title = "new Task";
        let mut task = Task::new(expected_title);
        task.complete();
        let expected_completed = task.completed_at;
        task.complete();
        assert_eq!(task.completed_at, expected_completed)
    }

}

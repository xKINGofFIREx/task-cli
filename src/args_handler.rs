use crate::task::Task;
use crate::task::Status;
use chrono::Utc;

pub fn add_task(description: &str, tasks: &mut Vec<Task>) {
    tasks.push(Task {
        id: tasks.len() as u64,
        description: description.to_string(),
        status: Status::InProgress,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });
}

pub fn update_task(id: &str, description: &str, tasks: &mut Vec<Task>) {
    let task = tasks.get_mut((id.parse::<u64>().expect("Failed to parse id argument") - 1) as usize).unwrap();
    task.description = description.to_string();
    task.updated_at = Utc::now();
}

pub fn delete_task(id: u64) {
    todo!()
}

pub fn list_tasks() {
    todo!()
}

pub fn list_done_tasks() {
    todo!()
}

pub fn list_inprogress_tasks() {
    todo!()
}

pub fn list_todo_tasks() {
    todo!()
}

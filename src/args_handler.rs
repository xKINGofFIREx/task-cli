use crate::task::Status;
use crate::task::Task;
use chrono::Utc;

pub fn add_task(description: &str, tasks: &mut Vec<Task>) {
    tasks.push(Task {
        description: description.to_string(),
        status: Status::InProgress,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });
}

pub fn update_task(id: &str, description: &str, tasks: &mut Vec<Task>) {
    let task = tasks
        .get_mut((id.parse::<u64>().expect("Failed to parse id argument") - 1) as usize)
        .unwrap();
    task.description = description.to_string();
    task.updated_at = Utc::now();
}

pub fn delete_task(id: &str, tasks: &mut Vec<Task>) {
    tasks.remove((id.parse::<u64>().expect("Failed to parse id argument") - 1) as usize);
}

pub fn list_tasks(tasks: &Vec<Task>) {
    for (id, task) in tasks.iter().enumerate() {
        println!("----------------------------------------------------");
        println!("Id: {}", id + 1);
        println!("Description: {}", task.description);
        println!("Status: {}", task.status);
        println!("Created at: {}", task.created_at);
        println!("Updated at: {}", task.updated_at);
        println!("----------------------------------------------------");
        println!();
    }
}

pub fn list_done_tasks(tasks: &Vec<Task>) {
    for (id, task) in tasks.iter().enumerate() {
        if let Status::Done = task.status {
            println!("----------------------------------------------------");
            println!("Id: {}", id + 1);
            println!("Description: {}", task.description);
            println!("Status: {}", task.status);
            println!("Created at: {}", task.created_at);
            println!("Updated at: {}", task.updated_at);
            println!("----------------------------------------------------");
            println!();
        }
    }
}

pub fn list_inprogress_tasks(tasks: &Vec<Task>) {
    for (id, task) in tasks.iter().enumerate() {
        if let Status::InProgress = task.status {
            println!("----------------------------------------------------");
            println!("Id: {}", id + 1);
            println!("Description: {}", task.description);
            println!("Status: {}", task.status);
            println!("Created at: {}", task.created_at);
            println!("Updated at: {}", task.updated_at);
            println!("----------------------------------------------------");
            println!();
        }
    }
}

pub fn list_todo_tasks(tasks: &Vec<Task>) {
    for (id, task) in tasks.iter().enumerate() {
        if let Status::Todo = task.status {
            println!("----------------------------------------------------");
            println!("Id: {}", id + 1);
            println!("Description: {}", task.description);
            println!("Status: {}", task.status);
            println!("Created at: {}", task.created_at);
            println!("Updated at: {}", task.updated_at);
            println!("----------------------------------------------------");
            println!();
        }
    }
}

pub fn mark_task_done(id: &str, tasks: &mut Vec<Task>) {
    let task = tasks
        .get_mut((id.parse::<u64>().expect("Failed to parse id argument") - 1) as usize)
        .unwrap();
    task.status = Status::Done;
}

pub fn mark_task_in_progress(id: &str, tasks: &mut Vec<Task>) {
    let task = tasks
        .get_mut((id.parse::<u64>().expect("Failed to parse id argument") - 1) as usize)
        .unwrap();
    task.status = Status::InProgress;
}

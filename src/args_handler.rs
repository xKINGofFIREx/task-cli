use crate::task::Status;
use crate::task::Task;
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
    let task = tasks
        .get_mut((id.parse::<u64>().expect("Failed to parse id argument") - 1) as usize)
        .unwrap();
    task.description = description.to_string();
    task.updated_at = Utc::now();
}

pub fn delete_task(id: &str, tasks: &mut Vec<Task>) {
    tasks.remove((id.parse::<u64>().expect("Failed to delete a task") - 1) as usize);
}

pub fn list_tasks(tasks: &mut Vec<Task>) {
    for task in tasks {
        print!("----------------------------------------------------");
        println!("{}", task.id);
        println!("{}", task.description);
        println!("{}", task.status);
        println!("{}", task.created_at);
        println!("{}", task.updated_at);
        print!("----------------------------------------------------");
        println!();
    }
}

pub fn list_done_tasks(tasks: &mut Vec<Task>) {
    for task in tasks {
        if let Status::Done = task.status {
            print!("----------------------------------------------------");
            println!("{}", task.id);
            println!("{}", task.description);
            println!("{}", task.status);
            println!("{}", task.created_at);
            println!("{}", task.updated_at);
            print!("----------------------------------------------------");
            println!();
        }
    }
}

pub fn list_inprogress_tasks(tasks: &mut Vec<Task>) {
    for task in tasks {
        if let Status::InProgress = task.status {
            print!("----------------------------------------------------");
            println!("{}", task.id);
            println!("{}", task.description);
            println!("{}", task.status);
            println!("{}", task.created_at);
            println!("{}", task.updated_at);
            print!("----------------------------------------------------");
            println!();
        }
    }
}

pub fn list_todo_tasks(tasks: &mut Vec<Task>) {
    for task in tasks {
        if let Status::Todo = task.status {
            print!("----------------------------------------------------");
            println!("{}", task.id);
            println!("{}", task.description);
            println!("{}", task.status);
            println!("{}", task.created_at);
            println!("{}", task.updated_at);
            print!("----------------------------------------------------");
            println!();
        }
    }
}

pub fn mark_task_done(id: &str, tasks: &mut Vec<Task>) {
    let task = tasks.get_mut((id.parse::<u64>().expect("Failed to parse id") - 1) as usize).unwrap();
    task.status = Status::Done;
}

pub fn mark_task_in_progress(id: &str, tasks: &mut Vec<Task>) {
    let task = tasks.get_mut((id.parse::<u64>().expect("Failed to parse id") - 1) as usize).unwrap();
    task.status = Status::InProgress;
}

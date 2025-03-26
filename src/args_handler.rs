use crate::task::Status;
use crate::task::Task;
use chrono::Utc;

pub fn add_task(description: &str, tasks: &mut Vec<Task>) {
    tasks.push(Task {
        description: description.to_string(),
        status: Status::Todo,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_task_to_provided_vec() {
        let mut tasks: Vec<Task> = Vec::new();
        let description = "test desc";

        add_task(description, &mut tasks);

        assert!(tasks.get(0).unwrap().description == description);
    }

    #[test]
    fn updates_existing_task() {
        let mut tasks: Vec<Task> = Vec::new();
        let description = "test desc";
        let new_description = "new description";

        add_task(description, &mut tasks);

        update_task("1", new_description, &mut tasks);

        assert!(tasks.get(0).unwrap().description == new_description);
    }

    #[test]
    fn deletes_existing_task() {
        let mut tasks: Vec<Task> = Vec::new();
        let description = "test desc";

        add_task(description, &mut tasks);

        assert_eq!(tasks.len(), 1);

        delete_task("1", &mut tasks);
        assert_eq!(tasks.len(), 0);
    }
}

pub mod args_handler;
pub mod task;

use crate::task::Task;
use serde_json;
use std::{env, fs};

fn main() {
    let mut tasks = load_tasks_data();

    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [cmd, description] if cmd == "add" => args_handler::add_task(description, &mut tasks),

        [cmd, id, description] if cmd == "update" => {
            args_handler::update_task(id, description, &mut tasks)
        }

        [cmd, id] if cmd == "delete" => args_handler::delete_task(id, &mut tasks),

        [cmd] if cmd == "list" => args_handler::list_tasks(&mut tasks),
        
        [cmd, status] if cmd == "list" && status == "done" => {
            args_handler::list_done_tasks(&mut tasks)
        }

        [cmd, status] if cmd == "list" && status == "todo" => {
            args_handler::list_todo_tasks(&mut tasks)
        }
        
        [cmd, status] if cmd == "list" && status == "in-progress" => {
            args_handler::list_inprogress_tasks(&mut tasks)
        }

        [cmd, id] if cmd == "mark-done" => args_handler::mark_task_done(id, &mut tasks),

        [cmd, id] if cmd == "mark-in-progress" => {
            args_handler::mark_task_in_progress(id, &mut tasks)
        }

        _ => eprintln!("Wrong arguments"),
    }

    std::fs::write("data.json", serde_json::to_string_pretty(&tasks).unwrap())
        .expect("Failed to write to data.json");
}

fn load_tasks_data() -> Vec<Task> {
    if let Ok(json_string) = fs::read_to_string("data.json") {
        return serde_json::from_str::<Vec<Task>>(&json_string).unwrap();
    } else {
        let _ = fs::File::create("data.json");
        return Vec::new();
    }
}

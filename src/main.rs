pub mod args_handler;
pub mod task;

use crate::task::Task;
use serde_json;
use std::{env, fs};

fn main() {
    let mut tasks = load_tasks_data();

    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        _ => eprintln!("Too many arguments"),
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

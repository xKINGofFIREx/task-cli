pub mod task;
pub mod args_handler;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        _ => eprintln!("Too many arguments"),
    }
}

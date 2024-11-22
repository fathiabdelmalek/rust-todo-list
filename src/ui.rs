use std::io;
use crate::task::Task;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn display_menu() {
    println!("1. Add Task");
    println!("2. View Tasks");
    println!("3. Mark Task as Completed");
    println!("4. Save Tasks");
    println!("5. Quit");
}

pub fn view_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        let status = if task.completed {"✓"} else {"✗"};
        println!("{}: [{}] {}", task.id, status, task.description);
    }
}

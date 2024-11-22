mod task;
mod storage;
mod ui;

use crate::task::Task;
use crate::storage::{load_task, save_tasks};
use crate::ui::{get_input, display_menu, view_tasks};

fn main() {
    println!("Welcome to Todo List Application!");
    let mut tasks: Vec<Task> = load_task().unwrap_or_else(|_| {
        println!("Could not load tasks from storage!");
        println!("Starting from an empty list");
        Vec::new()
    });

    loop {
        display_menu();
        let choice = get_input();
        match choice.as_str() {
            "1" => {
                println!("Enter the task description:");
                let description = get_input();
                println!("Enter the task description:");
                let priority: u8 = get_input().parse().unwrap();
                let id: u8 = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                tasks.push(Task::new(id, description, priority));
            }
            "2" => view_tasks(&mut tasks),
            "3" => {
                println!("Enter task number to mark as completed:");
                let id: u8 = get_input().parse().expect("Please enter a valid positive number.");
                if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                    task.completed = true;
                    println!("Task marked as completed");
                } else {
                    println!("Task not found");
                }
            }
            "4" => save_tasks(&tasks).expect("Could not save tasks!"),
            "5" => {
                save_tasks(&tasks).expect("Could not save tasks before exiting!");
                break;
            }
            _ => println!("Invalid choice, Please try again"),
        }
    }
}

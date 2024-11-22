use std::fs::{File, OpenOptions};
use std::path::Path;
use serde_json::Result as JsonResult;
use crate::task::Task;

pub fn load_task() -> JsonResult<Vec<Task>> {
    let path = "task.json";
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path).expect("Unable to open file");
    let tasks: Vec<Task> = serde_json::from_reader(file)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &Vec<Task>) -> JsonResult<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("task.json")
        .expect("Unable to open file");
    serde_json::to_writer(file, tasks)?;
    Ok(())
}

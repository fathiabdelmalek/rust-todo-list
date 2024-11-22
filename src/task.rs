use serde::{ Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u8,
    pub description: String,
    pub priority: u8,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u8, description: String, priority: u8) -> Task {
        Task {
            id,
            description,
            priority,
            completed: false,
        }
    }
}

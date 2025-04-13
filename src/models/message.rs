use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub content: String,
}

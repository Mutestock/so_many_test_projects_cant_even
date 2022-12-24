use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct NodeComment {
    content: String,
    date_added: DateTime<Local>,
}

impl NodeComment {
    pub fn new(content: String) -> Self {
        Self {
            content,
            date_added: Local::now(),
        }
    }
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::connection::{connection_common::MindmapConnector, sqlite_connection::SqliteConnection};

use super::model_common::ModelCommon;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum NodePreset {
    Event,
    Person,
    Document,
    Location,
    Appointment,
    Custom,
}

impl Display for NodePreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodePreset::Event => write!(f, "Event"),
            NodePreset::Person => write!(f, "Person"),
            NodePreset::Document => write!(f, "Document"),
            NodePreset::Location => write!(f, "Location"),
            NodePreset::Appointment => write!(f, "Appointment"),
            NodePreset::Custom => write!(f, "Custom"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeCategory {
    name: String,
}

impl ModelCommon<&str> for NodeCategory {
    fn init_script(&self, connector: SqliteConnection) {
        connector
        .connect()
        .expect("Could not create sqlite connection for NodeCategory init script")
        .execute("CREATE TABLE IF NOT EXISTS NodeCategory(category_name TEXT NOT NULL UNIQUE PRIMARY KEY);",
        ());
    }

    fn create(&self, connector: SqliteConnection) {
        todo!()
    }

    fn read(t: &str, connector: SqliteConnection) -> Self {
        todo!()
    }

    fn read_list(connector: SqliteConnection) -> Vec<Self> {
        todo!()
    }

    fn update(&self, t: &str, connector: SqliteConnection) {
        todo!()
    }

    fn delete(t: &str, connector: SqliteConnection) {
        todo!()
    }

    fn get_init_order(&self) -> u8 {
        todo!()
    }
}

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
    fn init_script(connector: &SqliteConnection) -> Result<(), rusqlite::Error> {
        connector
            .connect()?
            .execute(concat!(
                "CREATE TABLE IF NOT EXISTS NodeCategory(category_name TEXT NOT NULL UNIQUE PRIMARY KEY);",
                "INSERT OR IGNORE INTO NodeCategory (category_name) VALUES ('event'), ('person'), ('document'), ('location'), ('appointment');"
            ),
        ())?;

        Ok(())
    }

    fn create(&self, connector: &SqliteConnection) -> Result<(), rusqlite::Error> {
        connector.connect()?.execute(
            "INSERT INTO NodeCategory(category_name) VALUES (?1);",
            (&self.name,),
        )?;
        Ok(())
    }

    fn read(t: &str, connector: &SqliteConnection) -> Result<NodeCategory, rusqlite::Error> {
        todo!()
    }

    fn read_list(connector: &SqliteConnection) -> Vec<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn update(&self, t: &str, connector: &SqliteConnection) {
        todo!()
    }

    fn delete(t: &str, connector: &SqliteConnection) {
        todo!()
    }
}

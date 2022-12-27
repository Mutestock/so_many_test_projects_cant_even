use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::connection::{connection_common::MindmapConnector, sqlite_connection::SqliteConnector};

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
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS NodeCategory(
                category_name TEXT NOT NULL UNIQUE PRIMARY KEY
            );",
            (),
        )?;
        connection.execute(
            "INSERT OR IGNORE INTO NodeCategory (category_name) 
                VALUES ('event'), ('person'), ('document'), ('location'), ('appointment');",
            (),
        )?;
        Ok(())
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            "INSERT INTO NodeCategory(category_name) VALUES (?1);",
            (&self.name,),
        )?;
        Ok(())
    }

    fn read(t: &str, connection: &rusqlite::Connection) -> Result<NodeCategory, rusqlite::Error> {
        todo!()
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<NodeCategory>, rusqlite::Error>
    where
        Self: Sized,
    {
        todo!()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        todo!()
    }

    fn delete(t: &str, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        todo!()
    }

    fn from_row(p_key: Option<&str>, row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        todo!()
    }
}

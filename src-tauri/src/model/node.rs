use chrono::{DateTime, Local, TimeZone, FixedOffset};
use rusqlite::Row;
use serde::{Deserialize, Serialize};

use crate::connection::connection_common::MindmapConnector;
use crate::connection::sqlite_connection::SqliteConnection;
use crate::model::{model_common::ModelCommon};

use super::model_common::DateTimeRusqlite;


#[derive(Serialize, Deserialize)]
pub struct Node {
    name: String,
    date_added: DateTime<Local>,
    date_modified: DateTime<Local>,
    primary_image_path: Option<String>,
    node_category: String,
}

impl Node {
    pub fn new(name: String, node_category: String) -> Self {
        Self {
            name,
            date_added: Local::now(),
            date_modified: Local::now(),
            primary_image_path: None,
            node_category,
        }
    }

    pub fn as_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

impl ModelCommon<&str> for Node {
    fn init_script(connector: SqliteConnection) -> Result<(), rusqlite::Error> {
        connector.connect()?.execute(
            concat!(
                "CREATE TABLE IF NOT EXISTS Node(",
                "   name TEXT UNIQUE PRIMARY KEY NOT NULL,",
                "   date_added TEXT NOT NULL,",
                "   date_modified TEXT NOT NULL,",
                "   primary_image_path TEXT UNIQUE,",
                "   category_name TEXT NOT NULL,",
                "   FOREIGN KEY(category_name) REFERENCES NodeCategory(category_name)",
                ");",
            ),
            (),
        )?;
        Ok(())
    }

    fn create(&self, connector: SqliteConnection) -> Result<(), rusqlite::Error> {
        connector.connect()?.execute(
            concat!(
                "INSERT INTO Node (name, date_added, date_modified, category_name)",
                "VALUES (?1, ?2, ?3, ?4);"
            ),
            (
                &self.name,
                self.date_added.to_string(),
                self.date_modified.to_string(),
                &self.node_category,
            ),
        )?;
        Ok(())
    }

    fn read(t: &str, connector: SqliteConnection) -> Result<Node, rusqlite::Error> {
        let connection = connector
            .connect()?;
        let mut stmt = connection
            .prepare("SELECT date_added, date_modified, primary_image_path, category_name FROM Node WHERE name = ?1")?;

        let mut some_iter = stmt.query_map([t], |row|{
            Ok(
                Node {
                    name: t.to_owned(),
                    date_added: DateTime::<Local>::from_row(row, 0),
                    date_modified: DateTime::<Local>::from_row(row, 1),
                    primary_image_path: Some(row.get(2)?),
                    node_category: row.get(3)?, 
                }
            )
        })?;
        Ok(some_iter.next().unwrap()?)
    }

    fn read_list(connector: SqliteConnection) -> Vec<Node> {
        todo!()
    }

    fn update(&self, t: &str, connector: SqliteConnection) {
        todo!()
    }

    fn delete(t: &str, connector: SqliteConnection) {
        todo!()
    }
}

use crate::misc::time_management::{NaiveDateTimeRusqlite};

use super::model_common::ModelCommon;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct NodeComment {
    uuid: String,
    node_name: String,
    content: String,
    date_added: NaiveDateTime,
    date_modified: NaiveDateTime,
}

impl NodeComment {
    pub fn new(node_name: String, content: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            node_name,
            content,
            date_added: NaiveDateTime::now(),
            date_modified: NaiveDateTime::now(),
        }
    }
    pub fn uuid(&self) -> &str{
        &self.uuid
    }
}

impl ModelCommon<&str> for NodeComment {
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            concat!(
                "CREATE TABLE IF NOT EXISTS NodeComment (",
                "    uuid TEXT PRIMARY KEY NOT NULL UNIQUE,",
                "    content TEXT,",
                "    date_added TEXT NOT NULL,",
                "    date_modified TEXT,",
                "    node_name TEXT NOT NULL,",
                "    FOREIGN KEY(node_name) REFERENCES Node(name)",
                ");"
            ),
            (),
        )?;
        Ok(())
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            concat!(
                "INSERT INTO NodeComment(uuid, content, date_added, date_modified, node_name)",
                "VALUES( ?1, ?2, ?3, ?4, ?5"
            ),
            (
                &self.uuid,
                &self.content,
                self.date_added.to_string(),
                self.date_modified.to_string(),
                &self.node_name,
            ),
        )?;

        Ok(())
    }

    fn read(t: &str, connection: &rusqlite::Connection) -> Result<NodeComment, rusqlite::Error> {
        todo!()
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<NodeComment>, rusqlite::Error>
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
        Self: Sized {
        todo!()
    }
}

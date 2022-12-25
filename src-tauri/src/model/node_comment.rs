use super::model_common::ModelCommon;
use crate::connection::{connection_common::MindmapConnector, sqlite_connection::SqliteConnection};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct NodeComment {
    uuid: String,
    node_name: String,
    content: String,
    date_added: DateTime<Local>,
    date_modified: DateTime<Local>,
}

impl NodeComment {
    pub fn new(node_name: String, content: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            node_name,
            content,
            date_added: Local::now(),
            date_modified: Local::now(),
        }
    }
}

impl ModelCommon<&str> for NodeComment {
    fn init_script(connector: SqliteConnection) -> Result<(), rusqlite::Error> {
        connector.connect()?.execute(
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

    fn create(&self, connector: SqliteConnection) -> Result<(), rusqlite::Error> {
        connector.connect()?.execute(
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

    fn read(t: &str, connector: SqliteConnection) -> Result<NodeComment, rusqlite::Error> {
        todo!()
    }

    fn read_list(connector: SqliteConnection) -> Vec<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn update(&self, t: &str, connector: SqliteConnection) {
        todo!()
    }

    fn delete(t: &str, connector: SqliteConnection) {
        todo!()
    }
}

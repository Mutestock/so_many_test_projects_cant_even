use crate::connection::sqlite_connection::SqliteConnector;

use super::model_common::ModelCommon;

pub struct NodeImage {
    image_title: String,
    image_path: Option<String>,
    node_name: String,
}

impl NodeImage {
    pub fn new(image_title: String, node_name: String) -> Self {
        Self {
            image_title,
            image_path: None,
            node_name,
        }
    }
}

impl ModelCommon<&str> for NodeImage {
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        todo!()
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        todo!()
    }

    fn read(t: &str, connection: &rusqlite::Connection) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        todo!()
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<NodeImage>, rusqlite::Error>
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

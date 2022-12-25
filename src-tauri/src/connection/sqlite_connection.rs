use std::path::PathBuf;

use crate::connection::connection_common::MindmapConnector;
use rusqlite::{Connection, Error};

lazy_static! {
    pub static ref SQLITE_CONNECTION: SqliteConnection = SqliteConnection::default();
}

#[derive(Clone)]
pub struct SqliteConnection {
    database_file_path: Option<PathBuf>,
}

impl MindmapConnector<Connection, Error> for SqliteConnection {
    fn connect(&self) -> Result<Connection, Error> {
        Connection::open(&self.database_file_path.as_ref().unwrap())
    }

    fn get_dir_path(&self) -> &PathBuf {
        &self.database_file_path.as_ref().unwrap()
    }
}

impl SqliteConnection {
    fn new_in_memory(&self) -> Result<Connection, Error> {
        // For testing purposes
        Connection::open_in_memory()
    }
}

impl Default for SqliteConnection {
    fn default() -> Self {
        Self {
            database_file_path: Some(SqliteConnection::set_database_path("data/mindmap.sqlite")),
        }
    }
}

use std::path::PathBuf;

use crate::connection::connection_common::MindmapConnector;
use rusqlite::{Connection, Error};
use lazy_static;

lazy_static::lazy_static! {
    pub static ref SQLITE_CONNECTION: SqliteConnection = SqliteConnection::default();
}

#[derive(Clone)]
pub struct SqliteConnection {
    pub database_file_path: Option<PathBuf>,
}

impl MindmapConnector<Connection, Error> for SqliteConnection {
    fn connect(&self) -> Result<Connection, Error> {
        match &self.database_file_path.as_ref(){
            Some(db_file_path) => Connection::open(db_file_path),
            None => Connection::open_in_memory(),
        }
    }

    fn get_dir_path(&self) -> &PathBuf {
        &self.database_file_path.as_ref().unwrap()
    }
}


impl Default for SqliteConnection {
    fn default() -> Self {
        Self {
            database_file_path: Some(SqliteConnection::set_database_path("data/mindmap.sqlite")),
        }
    }
}

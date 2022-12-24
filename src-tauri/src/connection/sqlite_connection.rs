use std::path::PathBuf;

use crate::connection::connection_common::MindmapConnector;
use rusqlite::{Connection, Error};

lazy_static! {
    pub static ref SQLITE_CONNECTION: SqliteConnection = SqliteConnection::default();
}

pub struct SqliteConnection {
    database_file_path: PathBuf,
}

impl MindmapConnector<Connection, Error> for SqliteConnection {
    fn connect(&self) -> Result<Connection, Error> {
        Connection::open(&self.database_file_path)
    }

    fn get_dir_path(&self) -> &PathBuf {
        &self.database_file_path
    }
}

impl Default for SqliteConnection {
    fn default() -> Self {
        Self {
            database_file_path: SqliteConnection::set_database_path("data/mindmap.sqlite"),
        }
    }
}

pub fn init_pop() -> Result<(), Error> {
    println!(
        "sqlite path:   {}",
        SQLITE_CONNECTION.database_file_path.display()
    );

    let connection = SQLITE_CONNECTION
        .connect()
        .expect("Could not open connection");

    connection
        .execute(
            concat!(
                "CREATE TABLE IF NOT EXISTS Node(",
                "id INTEGER PRIMARY KEY AUTOINCREMENT,",
                "name TEXT NOT NULL",
                ");"
            ),
            (),
        )
        .expect("Init pop script failed");
    Ok(())
}

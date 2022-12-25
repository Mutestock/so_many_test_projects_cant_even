use lazy_static;
use mindmap::connection::sqlite_connection::SqliteConnection;


lazy_static::lazy_static! {
    pub static ref TESTING_SQLITE_CONNECTION : SqliteConnection = SqliteConnection{database_file_path: None};
}
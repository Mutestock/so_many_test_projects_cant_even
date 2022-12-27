use mindmap::connection::{sqlite_connection::SqliteConnector};


pub fn get_testing_environment() -> SqliteConnector{
    let sqlite_connection = SqliteConnector{database_file_path: None};
    sqlite_connection
}
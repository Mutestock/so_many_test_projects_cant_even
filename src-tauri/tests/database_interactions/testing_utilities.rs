use mindmap::connection::{sqlite_connection::SqliteConnection, initialize};


pub fn get_testing_environment() -> SqliteConnection{
    let sqlite_connection = SqliteConnection{database_file_path: None};
    initialize(&sqlite_connection).expect("Could not initialize table creation in testing");
    sqlite_connection
}
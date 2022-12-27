use mindmap::connection::{
    connection_common::MindmapConnector, initialize, sqlite_connection::SqliteConnector,
};

lazy_static::lazy_static! {
    pub static ref TESTING_SQLITE_CONNECTOR: SqliteConnector = get_testing_environment();
}

pub fn get_testing_environment() -> SqliteConnector {
    let sqlite_connection = SqliteConnector {
        database_file_path: None,
    };
    sqlite_connection
}

#[derive(Debug)]
struct Message(String);

#[test]
fn test_initialize() {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect().unwrap();
    initialize(&conn).expect("Could not initialize table creation in testing");
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';")
        .unwrap();

    let some_iter = stmt
        .query_map([], |row| Ok(Message(row.get(0).unwrap())))
        .unwrap();

    let stuff: Vec<Message> = some_iter.map(|x| x.unwrap()).collect();

    assert_eq!(stuff.len(), 3);
}

use mindmap::connection::{connection_common::MindmapConnector, initialize};
pub use mindmap::{
    connection::sqlite_connection::SqliteConnector,
    model::{model_common::ModelCommon, node::Node},
};
use rusqlite::Error;

use crate::database_interactions::testing_utilities::get_testing_environment;

lazy_static::lazy_static! {
    pub static ref TESTING_SQLITE_CONNECTOR: SqliteConnector = get_testing_environment();
}

#[derive(Debug)]
struct Message(String);

#[test]
fn test_create_node() -> Result<(), Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect().unwrap();
    initialize(&conn).expect("Could not initialize table creation in testing");
    Node::new("Cake".to_owned(), "event".to_owned())
        .create(&conn)
        .unwrap();
    Ok(())
}

#[test]
fn test_read_node() {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect().unwrap();
    initialize(&conn).expect("Could not initialize table creation in testing");
    Node::new("Cake".to_owned(), "event".to_owned())
        .create(&conn)
        .unwrap();

    let node = Node::read("Cake", &conn).unwrap();

    assert_eq!(node.node_category(), "event");
}

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

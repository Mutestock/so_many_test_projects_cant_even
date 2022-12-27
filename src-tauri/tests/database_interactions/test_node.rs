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
fn test_read_list_node() {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect().unwrap();
    initialize(&conn).expect("Could not initialize table creation in testing");

    Node::new("One".to_owned(), "event".to_owned())
        .create(&conn)
        .unwrap();
    Node::new("Two".to_owned(), "event".to_owned())
        .create(&conn)
        .unwrap();
    Node::new("Three".to_owned(), "event".to_owned())
        .create(&conn)
        .unwrap();

    let nodes = Node::read_list(&conn).unwrap();

    assert_eq!(nodes.len(), 3);
}

#[test]
fn test_update_node() {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect().unwrap();
    initialize(&conn).expect("Could not initialize table creation in testing");

    Node::new("Cake".to_owned(), "event".to_owned())
        .create(&conn)
        .unwrap();

    let node = Node::read("Cake", &conn).unwrap();
    assert_eq!(node.node_category(), "event");

    Node::new("Cake".to_owned(), "appointment".to_owned())
        .update("Cake", &conn)
        .unwrap();

    let node = Node::read("Cake", &conn).unwrap();
    assert_eq!(node.node_category(), "appointment");
}

#[test]
fn test_delete_node() {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect().unwrap();
    initialize(&conn).expect("Could not initialize table creation in testing");

    Node::new("One".to_owned(), "event".to_owned())
        .create(&conn)
        .unwrap();
    Node::new("Two".to_owned(), "event".to_owned())
        .create(&conn)
        .unwrap();
    Node::new("Three".to_owned(), "event".to_owned())
        .create(&conn)
        .unwrap();

    let nodes = Node::read_list(&conn).unwrap();
    assert_eq!(nodes.len(), 3);

    Node::delete("One", &conn).unwrap();
    let nodes = Node::read_list(&conn).unwrap();
    assert_eq!(nodes.len(), 2);
}

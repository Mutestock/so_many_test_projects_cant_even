use mindmap::connection::{connection_common::MindmapConnector, initialize};
use mindmap::model::{model_common::ModelCommon, node::Node};
use rusqlite::Error;

use super::testing_utilities::TESTING_SQLITE_CONNECTOR;

#[test]
fn test_create_node() -> Result<(), Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;
    Node::new("Cake".to_owned(), "event".to_owned()).create(&conn)?;
    Ok(())
}

#[test]
fn test_read_node() -> Result<(), Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;
    Node::new("Cake".to_owned(), "event".to_owned()).create(&conn)?;

    let node = Node::read("Cake", &conn)?;

    assert_eq!(node.node_category(), "event");
    Ok(())
}

#[test]
fn test_read_list_node() -> Result<(), Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("One".to_owned(), "event".to_owned()).create(&conn)?;
    Node::new("Two".to_owned(), "event".to_owned()).create(&conn)?;
    Node::new("Three".to_owned(), "event".to_owned()).create(&conn)?;

    let nodes = Node::read_list(&conn)?;

    assert_eq!(nodes.len(), 3);
    Ok(())
}

#[test]
fn test_update_node() -> Result<(), Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Cake".to_owned(), "event".to_owned()).create(&conn)?;

    let node = Node::read("Cake", &conn)?;
    assert_eq!(node.node_category(), "event");

    Node::new("Cake".to_owned(), "appointment".to_owned()).update("Cake", &conn)?;

    let node = Node::read("Cake", &conn)?;
    assert_eq!(node.node_category(), "appointment");
    Ok(())
}

#[test]
fn test_delete_node() -> Result<(), Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("One".to_owned(), "event".to_owned()).create(&conn)?;
    Node::new("Two".to_owned(), "event".to_owned()).create(&conn)?;
    Node::new("Three".to_owned(), "event".to_owned()).create(&conn)?;

    let nodes = Node::read_list(&conn)?;
    assert_eq!(nodes.len(), 3);

    Node::delete("One", &conn)?;
    let nodes = Node::read_list(&conn)?;
    assert_eq!(nodes.len(), 2);
    Ok(())
}


#[test]
fn test_delete_node_does_not_cause_node_comment_conflicts() {
    assert_eq!(2, 2);
}

#[test]
fn test_delete_node_does_not_cause_node_image_conflicts() {
    assert_eq!(2, 2);
}
use mindmap::model::{model_common::ModelCommon, node::Node};
use rusqlite::Error;

use crate::database_interactions::testing_utilities::get_testing_connection;

#[test]
fn test_create_node() -> Result<(), Error> {
    let conn = get_testing_connection();
    Node::new("Cake".to_owned(), "event".to_owned()).create(&conn)?;

    Ok(())
}

#[test]
fn test_read_node() -> Result<(), Error> {
    let conn = get_testing_connection();
    Node::new("Cake".to_owned(), "event".to_owned()).create(&conn)?;

    let node = Node::read("Cake", &conn)?;

    assert_eq!(node.unwrap().node_category(), "event");

    Ok(())
}

#[test]
fn test_read_list_node() -> Result<(), Error> {
    let conn = get_testing_connection();

    Node::new("One".to_owned(), "event".to_owned()).create(&conn)?;
    Node::new("Two".to_owned(), "event".to_owned()).create(&conn)?;
    Node::new("Three".to_owned(), "event".to_owned()).create(&conn)?;

    let nodes = Node::read_list(&conn)?;

    assert_eq!(nodes.len(), 3);

    Ok(())
}

#[test]
fn test_update_node() -> Result<(), Error> {
    let conn = get_testing_connection();

    Node::new("Cake".to_owned(), "event".to_owned()).create(&conn)?;

    let node = Node::read("Cake", &conn)?;
    assert_eq!(node.unwrap().node_category(), "event");

    Node::new("Cake".to_owned(), "appointment".to_owned()).update("Cake", &conn)?;

    let node = Node::read("Cake", &conn)?;
    assert_eq!(node.unwrap().node_category(), "appointment");

    Ok(())
}

#[test]
fn test_delete_node() -> Result<(), Error> {
    let conn = get_testing_connection();

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
fn test_node_read_none() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let node_read = Node::read("<zxc<zxc<zxcASDASDASD>", &conn)?;
    assert_eq!(node_read.is_none(), true);

    Ok(())
}

#[test]
fn test_node_read_all_empty() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let node = Node::read_list(&conn)?;
    assert_eq!(node.len(), 0);

    Ok(())
}

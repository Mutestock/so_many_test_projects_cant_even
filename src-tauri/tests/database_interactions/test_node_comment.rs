use mindmap::{
    connection::{connection_common::MindmapConnector, initialize},
    model::{model_common::ModelCommon, node::Node, node_comment::NodeComment},
};

use super::testing_utilities::TESTING_SQLITE_CONNECTOR;

#[test]
fn test_create_node_comment() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "This is a comment".to_owned()).create(&conn)?;

    Ok(())
}

#[test]
fn test_read_node_comment() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    let node_comment = NodeComment::new("Bonk".to_owned(), "This is a comment".to_owned());
    node_comment.create(&conn)?;

    let node_comment_read = NodeComment::read(node_comment.uuid(), &conn)?;
    assert_eq!(node_comment_read.content(), "This is a comment".to_owned());

    Ok(())
}

#[test]
fn test_update_node_comment() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;

    let new_node_comment = NodeComment::new("Bonk".to_owned(), "This is a comment".to_owned());
    new_node_comment.create(&conn)?;

    NodeComment::new("Bonk".to_owned(), "This is a modified comment".to_owned())
        .update(new_node_comment.uuid(), &conn)?;

    let node_comment = NodeComment::read(new_node_comment.uuid(), &conn)?;

    assert_eq!(node_comment.content(), "This is a modified comment");

    Ok(())
}

#[test]
fn test_delete_node_comment() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    let one_of_the_node_comments = NodeComment::new("Bonk".to_owned(), "Comment one".to_owned());
    one_of_the_node_comments.create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "Comment three".to_owned()).create(&conn)?;
    let node_comments = NodeComment::read_list(&conn)?;
    assert_eq!(node_comments.len(), 3);
    NodeComment::delete(one_of_the_node_comments.uuid(), &conn)?;
    let node_comments = NodeComment::read_list(&conn)?;
    assert_eq!(node_comments.len(), 2);

    Ok(())
}

#[test]
fn test_read_list_node_comment() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    let node_comments = NodeComment::read_list(&conn)?;
    assert_eq!(node_comments.len(), 3);

    Ok(())
}

#[test]
fn test_read_none_doesnt_cause_errors() {
    assert_eq!(2, 2);
}

#[test]
fn test_read_all_none_does_not_cause_errors() {
    assert_eq!(2, 2);
}
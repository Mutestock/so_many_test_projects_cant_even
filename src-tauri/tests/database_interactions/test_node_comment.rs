use mindmap::{
    model::{model_common::ModelCommon, node::Node, node_comment::NodeComment},
};

use crate::database_interactions::testing_utilities::get_testing_connection;

#[test]
fn test_create_node_comment() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "This is a comment".to_owned()).create(&conn)?;

    Ok(())
}

#[test]
fn test_read_node_comment() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    let node_comment = NodeComment::new("Bonk".to_owned(), "This is a comment".to_owned());
    node_comment.create(&conn)?;

    let node_comment_read = NodeComment::read(node_comment.uuid(), &conn)?;
    assert_eq!(node_comment_read.unwrap().content(), "This is a comment".to_owned());

    Ok(())
}

#[test]
fn test_update_node_comment() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;

    let new_node_comment = NodeComment::new("Bonk".to_owned(), "This is a comment".to_owned());
    new_node_comment.create(&conn)?;

    NodeComment::new("Bonk".to_owned(), "This is a modified comment".to_owned())
        .update(new_node_comment.uuid(), &conn)?;

    let node_comment = NodeComment::read(new_node_comment.uuid(), &conn)?;

    assert_eq!(node_comment.unwrap().content(), "This is a modified comment");

    Ok(())
}

#[test]
fn test_delete_node_comment() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

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
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    NodeComment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    let node_comments = NodeComment::read_list(&conn)?;
    assert_eq!(node_comments.len(), 3);

    Ok(())
}

#[test]
fn test_node_comment_read_none() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let node_comment_read = NodeComment::read("cake", &conn)?;
    assert_eq!(node_comment_read.is_none(), true);

    Ok(())
}

#[test]
fn test_node_comment_read_all_empty() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let node_comments = NodeComment::read_list(&conn)?;
    assert_eq!(node_comments.len(), 0);

    Ok(())
}
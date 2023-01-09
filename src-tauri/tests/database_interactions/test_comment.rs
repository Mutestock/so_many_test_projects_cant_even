use mindmap::{
    model::{model_common::ModelCommon, node::Node, comment::Comment},
};

use crate::database_interactions::testing_utilities::get_testing_connection;

#[test]
fn test_create_comment() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    Comment::new("Bonk".to_owned(), "This is a comment".to_owned()).create(&conn)?;

    Ok(())
}

#[test]
fn test_read_comment() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    let comment = Comment::new("Bonk".to_owned(), "This is a comment".to_owned());
    comment.create(&conn)?;

    let comment_read = Comment::read(comment.uuid(), &conn)?;
    assert_eq!(comment_read.unwrap().content(), "This is a comment".to_owned());

    Ok(())
}

#[test]
fn test_update_comment() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;

    let new_comment = Comment::new("Bonk".to_owned(), "This is a comment".to_owned());
    new_comment.create(&conn)?;

    Comment::new("Bonk".to_owned(), "This is a modified comment".to_owned())
        .update(new_comment.uuid(), &conn)?;

    let comment = Comment::read(new_comment.uuid(), &conn)?;

    assert_eq!(comment.unwrap().content(), "This is a modified comment");

    Ok(())
}

#[test]
fn test_delete_comment() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    let one_of_the_comments = Comment::new("Bonk".to_owned(), "Comment one".to_owned());
    one_of_the_comments.create(&conn)?;
    Comment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    Comment::new("Bonk".to_owned(), "Comment three".to_owned()).create(&conn)?;
    let comments = Comment::read_list(&conn)?;
    assert_eq!(comments.len(), 3);
    Comment::delete(one_of_the_comments.uuid(), &conn)?;
    let comments = Comment::read_list(&conn)?;
    assert_eq!(comments.len(), 2);

    Ok(())
}

#[test]
fn test_read_list_comment() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    Comment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    Comment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    Comment::new("Bonk".to_owned(), "Comment two".to_owned()).create(&conn)?;
    let comments = Comment::read_list(&conn)?;
    assert_eq!(comments.len(), 3);

    Ok(())
}

#[test]
fn test_comment_read_none() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let comment_read = Comment::read("cake", &conn)?;
    assert_eq!(comment_read.is_none(), true);

    Ok(())
}

#[test]
fn test_comment_read_all_empty() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let comments = Comment::read_list(&conn)?;
    assert_eq!(comments.len(), 0);

    Ok(())
}
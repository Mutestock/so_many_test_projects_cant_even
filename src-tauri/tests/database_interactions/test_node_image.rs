use mindmap::model::{model_common::ModelCommon, node::Node, node_image::NodeImage};

use crate::database_interactions::testing_utilities::{cleanup, get_testing_connection};

#[test]
fn test_create_node_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeImage::new("New Image".to_owned(), "Bonk".to_owned()).create(&conn)?;

    cleanup();

    Ok(())
}

#[test]
fn test_read_node_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeImage::new("New Image".to_owned(), "Bonk".to_owned()).create(&conn)?;

    let node_image = NodeImage::read("New Image", &conn)?;
    assert_eq!(node_image.unwrap().node_name(), "Bonk".to_owned());

    cleanup();

    Ok(())
}

#[test]
fn test_update_node_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeImage::new("New Image".to_owned(), "Bonk".to_owned()).create(&conn)?;
    NodeImage::new("Modified Image".to_owned(), "Bonk".to_owned()).update("New Image", &conn)?;

    let node_image = NodeImage::read("Modified Image", &conn)?;
    assert_eq!(
        node_image.unwrap().image_title(),
        "Modified Image".to_owned()
    );

    cleanup();

    Ok(())
}

#[test]
fn test_delete_node_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;

    NodeImage::new("one".to_owned(), "Bonk".to_owned()).create(&conn)?;
    NodeImage::new("two".to_owned(), "Bonk".to_owned()).create(&conn)?;
    NodeImage::new("three".to_owned(), "Bonk".to_owned()).create(&conn)?;
    let node_images = NodeImage::read_list(&conn)?;
    assert_eq!(node_images.len(), 3);
    NodeImage::delete("two", &conn)?;
    let node_images = NodeImage::read_list(&conn)?;
    assert_eq!(node_images.len(), 2);

    cleanup();

    Ok(())
}

#[test]
fn test_read_list_node_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeImage::new("one".to_owned(), "Bonk".to_owned()).create(&conn)?;
    NodeImage::new("two".to_owned(), "Bonk".to_owned()).create(&conn)?;
    NodeImage::new("three".to_owned(), "Bonk".to_owned()).create(&conn)?;
    let node_images = NodeImage::read_list(&conn)?;
    assert_eq!(node_images.len(), 3);

    cleanup();

    Ok(())
}

#[test]
fn test_node_image_read_none() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let node_image_read = NodeImage::read("cake", &conn)?;
    assert_eq!(node_image_read.is_none(), true);

    Ok(())
}

#[test]
fn test_node_image_read_all_empty() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let node_images = NodeImage::read_list(&conn)?;
    assert_eq!(node_images.len(), 0);

    Ok(())
}

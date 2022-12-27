use mindmap::{
    connection::{connection_common::MindmapConnector, initialize},
    model::{model_common::ModelCommon, node::Node, node_image::NodeImage},
};

use super::testing_utilities::TESTING_SQLITE_CONNECTOR;

#[test]
fn test_create_node_image() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeImage::new("New Image".to_owned(), "Bonk".to_owned()).create(&conn)?;

    Ok(())
}

#[test]
fn test_read_node_image() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeImage::new("New Image".to_owned(), "Bonk".to_owned()).create(&conn)?;

    let node_image = NodeImage::read("New Image", &conn)?;
    assert_eq!(node_image.node_name(), "This is a image".to_owned());

    Ok(())
}

#[test]
fn test_update_node_image() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeImage::new("New Image".to_owned(), "Bonk".to_owned()).create(&conn)?;

    NodeImage::new("Modified Image".to_owned(), "Bonk".to_owned()).update("New Image", &conn)?;

    let node_image = NodeImage::read("Modified Image", &conn)?;
    assert_eq!(node_image.image_title(), "Modified Image".to_owned());

    Ok(())
}

#[test]
fn test_delete_node_image() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;

    NodeImage::new("one".to_owned(), "Bonk".to_owned()).create(&conn)?;
    NodeImage::new("two".to_owned(), "Bonk".to_owned()).create(&conn)?;
    NodeImage::new("three".to_owned(), "Bonk".to_owned()).create(&conn)?;
    let node_images = NodeImage::read_list(&conn)?;
    assert_eq!(node_images.len(), 3);
    NodeImage::delete("two", &conn)?;
    assert_eq!(node_images.len(), 2);

    Ok(())
}

#[test]
fn test_read_list_node_image() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    NodeImage::new("one".to_owned(), "Bonk".to_owned()).create(&conn)?;
    NodeImage::new("two".to_owned(), "Bonk".to_owned()).create(&conn)?;
    NodeImage::new("three".to_owned(), "Bonk".to_owned()).create(&conn)?;
    let node_images = NodeImage::read_list(&conn)?;
    assert_eq!(node_images.len(), 3);

    Ok(())
}

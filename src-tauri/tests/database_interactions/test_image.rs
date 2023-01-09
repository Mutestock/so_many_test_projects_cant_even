use mindmap::model::{model_common::ModelCommon, node::Node, image::Image};

use crate::database_interactions::testing_utilities::{cleanup, get_testing_connection};

#[test]
fn test_create_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    Image::new("New Image".to_owned(), "Bonk".to_owned()).create(&conn)?;

    cleanup();

    Ok(())
}

#[test]
fn test_read_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    Image::new("New Image".to_owned(), "Bonk".to_owned()).create(&conn)?;

    let image = Image::read("New Image", &conn)?;
    assert_eq!(image.unwrap().node_name(), "Bonk".to_owned());

    cleanup();

    Ok(())
}

#[test]
fn test_update_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    Image::new("New Image".to_owned(), "Bonk".to_owned()).create(&conn)?;
    Image::new("Modified Image".to_owned(), "Bonk".to_owned()).update("New Image", &conn)?;

    let image = Image::read("Modified Image", &conn)?;
    assert_eq!(
        image.unwrap().image_title(),
        "Modified Image".to_owned()
    );

    cleanup();

    Ok(())
}

#[test]
fn test_delete_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;

    Image::new("one".to_owned(), "Bonk".to_owned()).create(&conn)?;
    Image::new("two".to_owned(), "Bonk".to_owned()).create(&conn)?;
    Image::new("three".to_owned(), "Bonk".to_owned()).create(&conn)?;
    let images = Image::read_list(&conn)?;
    assert_eq!(images.len(), 3);
    Image::delete("two", &conn)?;
    let images = Image::read_list(&conn)?;
    assert_eq!(images.len(), 2);

    cleanup();

    Ok(())
}

#[test]
fn test_read_list_image() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    Node::new("Bonk".to_owned(), "event".to_owned()).create(&conn)?;
    Image::new("one".to_owned(), "Bonk".to_owned()).create(&conn)?;
    Image::new("two".to_owned(), "Bonk".to_owned()).create(&conn)?;
    Image::new("three".to_owned(), "Bonk".to_owned()).create(&conn)?;
    let images = Image::read_list(&conn)?;
    assert_eq!(images.len(), 3);

    cleanup();

    Ok(())
}

#[test]
fn test_image_read_none() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let image_read = Image::read("cake", &conn)?;
    assert_eq!(image_read.is_none(), true);

    Ok(())
}

#[test]
fn test_image_read_all_empty() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let images = Image::read_list(&conn)?;
    assert_eq!(images.len(), 0);

    Ok(())
}

#[test]
fn test_delete_by_node_name() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    todo!();
}
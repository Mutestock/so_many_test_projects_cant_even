use mindmap::{
    connection::{connection_common::MindmapConnector, initialize},
    model::{model_common::ModelCommon, node_category::NodeCategory},
};

use super::testing_utilities::TESTING_SQLITE_CONNECTOR;

#[test]
fn test_create_node_category() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    NodeCategory::new("junk".to_owned()).create(&conn)?;

    Ok(())
}

#[test]
fn test_read_node_category() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;
    let node_category = NodeCategory::read("event", &conn)?;
    assert_eq!(node_category.name(), "event");

    Ok(())
}

#[test]
fn test_update_node_category() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;
    NodeCategory::new("flerp".to_owned()).update("event", &conn)?;
    let node_category = NodeCategory::read("flerp", &conn)?;
    assert_eq!(node_category.name(), "flerp");

    Ok(())
}

#[test]
fn test_delete_node_category() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;

    assert_eq!(NodeCategory::read_list(&conn)?.len(), 6);
    NodeCategory::delete("event", &conn)?;
    assert_eq!(NodeCategory::read_list(&conn)?.len(), 5);
    Ok(())
}

#[test]
fn test_read_list_node_category() -> Result<(), rusqlite::Error> {
    let conn = TESTING_SQLITE_CONNECTOR.to_owned().connect()?;
    initialize(&conn)?;
    assert_eq!(NodeCategory::read_list(&conn)?.len(), 6);
    Ok(())
}


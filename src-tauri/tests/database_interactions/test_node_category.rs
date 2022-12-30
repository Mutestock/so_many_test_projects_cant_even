use mindmap::model::{model_common::ModelCommon, node_category::NodeCategory};

use crate::database_interactions::testing_utilities::get_testing_connection;

#[test]
fn test_create_node_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    NodeCategory::new("junk".to_owned()).create(&conn)?;

    Ok(())
}

#[test]
fn test_read_node_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    let node_category = NodeCategory::read("event", &conn)?;
    assert_eq!(node_category.unwrap().name(), "event");

    Ok(())
}

#[test]
fn test_update_node_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    NodeCategory::new("flerp".to_owned()).update("event", &conn)?;
    let node_category = NodeCategory::read("flerp", &conn)?;
    assert_eq!(node_category.unwrap().name(), "flerp");

    Ok(())
}

#[test]
fn test_delete_node_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    assert_eq!(NodeCategory::read_list(&conn)?.len(), 6);
    NodeCategory::delete("event", &conn)?;
    assert_eq!(NodeCategory::read_list(&conn)?.len(), 5);
    Ok(())
}

#[test]
fn test_read_list_node_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    assert_eq!(NodeCategory::read_list(&conn)?.len(), 6);
    Ok(())
}

#[test]
fn test_node_category_read_none() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let node_comment_read = NodeCategory::read("asphyxia", &conn)?;
    assert_eq!(node_comment_read.is_none(), true);

    Ok(())
}

#[test]
fn test_node_category_read_all_empty() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    conn.prepare("DELETE FROM NodeCategory")
        .unwrap()
        .execute([])
        .unwrap();

    let node_categories = NodeCategory::read_list(&conn)?;
    assert_eq!(node_categories.len(), 0);

    Ok(())
}

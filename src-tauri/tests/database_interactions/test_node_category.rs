use mindmap::model::{
    model_common::ModelCommon,
    node_category::{NodeCategory, DEFAULT_CATEGORIES},
};

use crate::database_interactions::testing_utilities::get_testing_connection;

#[test]
fn test_create_node_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    NodeCategory::new("junk".to_owned(), "D132C6".to_owned()).create(&conn)?;

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
    NodeCategory::new("flerp".to_owned(), "#F52020".to_owned()).update("event", &conn)?;
    let node_category = NodeCategory::read("flerp", &conn)?;
    assert_eq!(node_category.unwrap().name(), "flerp");

    Ok(())
}

#[test]
fn test_delete_node_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    assert_eq!(
        NodeCategory::read_list(&conn)?.len(),
        DEFAULT_CATEGORIES.len()
    );
    NodeCategory::delete("event", &conn)?;
    assert_eq!(
        NodeCategory::read_list(&conn)?.len(),
        DEFAULT_CATEGORIES.len() - 1
    );
    Ok(())
}

#[test]
fn test_read_list_node_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    assert_eq!(
        NodeCategory::read_list(&conn)?.len(),
        DEFAULT_CATEGORIES.len()
    );
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

#[test]
fn test_color_code_hex_is_valid() {
    assert_eq!(true, NodeCategory::is_valid_hex("#F52020"));
    assert_eq!(true, NodeCategory::is_valid_hex("#000000"));
    assert_eq!(true, NodeCategory::is_valid_hex("#FFFFFF"));
    assert_eq!(true, NodeCategory::is_valid_hex("#2F7D29"));
    assert_eq!(true, NodeCategory::is_valid_hex("#CFF"));
}

#[test]
fn test_bad_color_codes_are_invalid() {
    assert_eq!(false, NodeCategory::is_valid_hex("#ZEEFQ2"));
    assert_eq!(false, NodeCategory::is_valid_hex("#GEE"));
    assert_eq!(false, NodeCategory::is_valid_hex("#CAAAAAAKE"));
    assert_eq!(false, NodeCategory::is_valid_hex("#0ADzAb"));
    assert_eq!(false, NodeCategory::is_valid_hex("#BEP"));
}

#[test]
fn test_category_toggle_visibility() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    let category = NodeCategory::read("event", &conn)?.unwrap();
    assert_eq!(category.visibility_toggled_on, true);

    NodeCategory::update_category_toggle_visisbility("event", &conn)?;
    let category = NodeCategory::read("event", &conn)?.unwrap();
    assert_eq!(category.visibility_toggled_on, false);
    
    NodeCategory::update_category_toggle_visisbility("event", &conn)?;
    let category = NodeCategory::read("event", &conn)?.unwrap();
    assert_eq!(category.visibility_toggled_on, true);

    Ok(())
}

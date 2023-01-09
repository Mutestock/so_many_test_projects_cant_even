use mindmap::model::{
    model_common::ModelCommon,
    category::{Category, DEFAULT_CATEGORIES},
};

use crate::database_interactions::testing_utilities::get_testing_connection;

#[test]
fn test_create_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    Category::new("junk".to_owned(), "D132C6".to_owned()).create(&conn)?;

    Ok(())
}

#[test]
fn test_read_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    let category = Category::read("event", &conn)?;
    assert_eq!(category.unwrap().name(), "event");

    Ok(())
}

#[test]
fn test_update_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    Category::new("flerp".to_owned(), "#F52020".to_owned()).update("event", &conn)?;
    let category = Category::read("flerp", &conn)?;
    assert_eq!(category.unwrap().name(), "flerp");

    Ok(())
}

#[test]
fn test_delete_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    assert_eq!(
        Category::read_list(&conn)?.len(),
        DEFAULT_CATEGORIES.len()
    );
    Category::delete("event", &conn)?;
    assert_eq!(
        Category::read_list(&conn)?.len(),
        DEFAULT_CATEGORIES.len() - 1
    );
    Ok(())
}

#[test]
fn test_read_list_category() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    assert_eq!(
        Category::read_list(&conn)?.len(),
        DEFAULT_CATEGORIES.len()
    );
    Ok(())
}

#[test]
fn test_category_read_none() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    let comment_read = Category::read("asphyxia", &conn)?;
    assert_eq!(comment_read.is_none(), true);

    Ok(())
}

#[test]
fn test_category_read_all_empty() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();

    conn.prepare("DELETE FROM Category")
        .unwrap()
        .execute([])
        .unwrap();

    let node_categories = Category::read_list(&conn)?;
    assert_eq!(node_categories.len(), 0);

    Ok(())
}

#[test]
fn test_color_code_hex_is_valid() {
    assert_eq!(true, Category::is_valid_hex("#F52020"));
    assert_eq!(true, Category::is_valid_hex("#000000"));
    assert_eq!(true, Category::is_valid_hex("#FFFFFF"));
    assert_eq!(true, Category::is_valid_hex("#2F7D29"));
    assert_eq!(true, Category::is_valid_hex("#CFF"));
}

#[test]
fn test_bad_color_codes_are_invalid() {
    assert_eq!(false, Category::is_valid_hex("#ZEEFQ2"));
    assert_eq!(false, Category::is_valid_hex("#GEE"));
    assert_eq!(false, Category::is_valid_hex("#CAAAAAAKE"));
    assert_eq!(false, Category::is_valid_hex("#0ADzAb"));
    assert_eq!(false, Category::is_valid_hex("#BEP"));
}

#[test]
fn test_category_toggle_visibility() -> Result<(), rusqlite::Error> {
    let conn = get_testing_connection();
    let category = Category::read("event", &conn)?.unwrap();
    assert_eq!(category.visibility_toggled_on, true);

    Category::update_category_toggle_visisbility("event", &conn)?;
    let category = Category::read("event", &conn)?.unwrap();
    assert_eq!(category.visibility_toggled_on, false);

    Category::update_category_toggle_visisbility("event", &conn)?;
    let category = Category::read("event", &conn)?.unwrap();
    assert_eq!(category.visibility_toggled_on, true);

    Ok(())
}

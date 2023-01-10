use mindmap::model::{model_common::ModelCommon, tag::Tag};
use rusqlite::Error;

use crate::database_interactions::testing_utilities::get_testing_connection;

#[test]
fn test_create_tag() -> Result<(), Error> {
    let conn = get_testing_connection();
    Tag::new("some_tag".to_owned()).create(&conn)?;

    Ok(())
}

#[test]
fn test_read_list_tag() -> Result<(), Error> {
    let conn = get_testing_connection();

    Tag::new("tag_one".to_owned()).create(&conn)?;
    Tag::new("tag_two".to_owned()).create(&conn)?;
    Tag::new("tag_three".to_owned()).create(&conn)?;

    let tag_list = Tag::read_list(&conn)?;

    assert_eq!(tag_list.len(), 3);

    Ok(())
}

#[test]
fn test_update_tag() -> Result<(), Error> {
    let conn = get_testing_connection();
    Tag::new("some_tag".to_owned()).create(&conn)?;
    let tag_read1 = Tag::read_list(&conn)?.swap_remove(0);
    assert_eq!(tag_read1.name, "some_tag");

    Tag::new("other_tag".to_owned()).update("some_tag", &conn)?;

    let mut tag_list = Tag::read_list(&conn)?;
    assert_eq!(tag_list.len(), 1);
    assert_eq!(tag_list.swap_remove(0).name, "other_tag");

    Ok(())
}

#[test]
fn test_delete_tag() -> Result<(), Error> {
    let conn = get_testing_connection();
    Tag::new("tag_one".to_owned()).create(&conn)?;
    Tag::new("tag_two".to_owned()).create(&conn)?;
    Tag::new("tag_three".to_owned()).create(&conn)?;
    let tag_list = Tag::read_list(&conn)?;

    assert_eq!(tag_list.len(), 3);

    Tag::delete("tag_two", &conn)?;
    let tag_list = Tag::read_list(&conn)?;
    assert_eq!(tag_list.len(), 2);

    Ok(())
}

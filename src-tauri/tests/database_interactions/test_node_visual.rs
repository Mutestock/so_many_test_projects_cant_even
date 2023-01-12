use mindmap::model::{node::Node, category::Category, hybrids::node_visual::NodeVisual, model_common::ModelCommon};

use crate::database_interactions::testing_utilities::get_testing_connection;

#[test]
fn test_read_list_toggled_on() -> Result<(), rusqlite::Error>{
    let conn = get_testing_connection();

    Node::new("One".to_owned(), "event".to_owned()).create(&conn)?;
    Node::new("Two".to_owned(), "bill".to_owned()).create(&conn)?;
    Node::new("Three".to_owned(), "person".to_owned()).create(&conn)?;
    Node::new("Four".to_owned(), "person".to_owned()).create(&conn)?;

    Category::update_category_toggle_visisbility("event", &conn)?;
    Category::update_category_toggle_visisbility("bill", &conn)?;

    let node_visuals = NodeVisual::read_list_toggled_on(&conn)?;

    assert_eq!(node_visuals.len(), 2);

    Ok(())
}
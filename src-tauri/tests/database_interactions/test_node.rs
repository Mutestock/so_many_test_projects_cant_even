pub use mindmap::{connection::sqlite_connection::SqliteConnection, model::{node::Node, model_common::ModelCommon}};

use crate::database_interactions::testing_utilities::get_testing_environment;





lazy_static::lazy_static! {
    pub static ref TESTING_SQLITE_CONNECTION: SqliteConnection = get_testing_environment();
}


#[test]
fn test_create_node() {
    Node::new("Cake".to_owned(), "event".to_owned())
        .create(&TESTING_SQLITE_CONNECTION)
        .unwrap();
}

#[test]
fn test_read_node() {
    Node::new("Cake".to_owned(), "event".to_owned())
        .create(&TESTING_SQLITE_CONNECTION)
        .unwrap();
    
    let node = Node::read("Cake", &TESTING_SQLITE_CONNECTION).unwrap();

    assert_eq!(node.)
}

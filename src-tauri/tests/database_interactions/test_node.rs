use mindmap::connection::{connection_common::MindmapConnector, initialize};
pub use mindmap::{
    connection::sqlite_connection::SqliteConnection,
    model::{model_common::ModelCommon, node::Node},
};

use crate::database_interactions::testing_utilities::get_testing_environment;

lazy_static::lazy_static! {
    pub static ref TESTING_SQLITE_CONNECTION: SqliteConnection = get_testing_environment();
}

#[test]
fn test_create_node() {
    let sqlite_connection = SqliteConnection {
        database_file_path: None,
    };
    initialize(&sqlite_connection).expect("Could not initialize table creation in testing");
    let stmt = sqlite_connection.connect().unwrap().prepare(sql_query()).unwrap();
    let mut some_iter = stmt.query_map([], |row| {
        println!("{}", row.get(0));
    }).unwrap();

    let rows = sqlite_connection.connect().unwrap().query_row(
        sql_query(),
        (),
    );


    Node::new("Cake".to_owned(), "event".to_owned())
        .create(&TESTING_SQLITE_CONNECTION)
        .unwrap();
}

fn sql_query() -> &str {
    r#"SELECT 
    name
FROM 
    sqlite_schema
WHERE 
    type ='table' AND 
    name NOT LIKE 'sqlite_%';"#
}

#[test]
fn test_read_node() {
    Node::new("Cake".to_owned(), "event".to_owned())
        .create(&TESTING_SQLITE_CONNECTION)
        .unwrap();

    let node = Node::read("Cake", &TESTING_SQLITE_CONNECTION).unwrap();

    assert_eq!(node.node_category(), "event");
}

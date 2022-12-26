use mindmap::connection::{connection_common::MindmapConnector, initialize};
pub use mindmap::{
    connection::sqlite_connection::SqliteConnection,
    model::{model_common::ModelCommon, node::Node},
};
use rusqlite::{Connection, Error};

use crate::database_interactions::testing_utilities::get_testing_environment;

lazy_static::lazy_static! {
    pub static ref TESTING_SQLITE_CONNECTION: SqliteConnection = get_testing_environment();
}

#[derive(Debug)]
struct Message(String);

#[test]
fn test_create_node() -> Result<(), Error> {
    let conn = TESTING_SQLITE_CONNECTION.to_owned().connect().unwrap();
    let query = "
    CREATE TABLE Node(
        name TEXT UNIQUE PRIMARY KEY NOT NULL,
        date_added TEXT NOT NULL,
        date_modified TEXT NOT NULL,
        primary_image_path TEXT UNIQUE,
        category_name TEXT NOT NULL,
        FOREIGN KEY(category_name) REFERENCES NodeCategory(category_name)
    )";

    conn.execute("CREATE TABLE IF NOT EXISTS NodeCategory(category_name TEXT NOT NULL UNIQUE PRIMARY KEY);",())?;
    conn.execute("INSERT OR IGNORE INTO NodeCategory (category_name) VALUES ('event'), ('person'), ('document'), ('location'), ('appointment');",())?;
    conn.execute(query, ())?;
    
    //initialize(&sqlite_connection).expect("Could not initialize table creation in testing");
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';")
        .unwrap();

    let some_iter = stmt
        .query_map([], |row| Ok(Message(row.get(0).unwrap())))
        .unwrap();

    let stuff: Vec<Message> = some_iter.map(|x| x.unwrap()).collect();

    assert_eq!(stuff.len(), 2);

    for something in stuff.iter() {
        println!("{:?}", something);
    }
    let node = Node::new("Cake".to_owned(), "event".to_owned());

    conn.execute(
        concat!(
            "INSERT INTO Node (name, date_added, date_modified, category_name)",
            "VALUES (?1, ?2, ?3, ?4);"
        ),
        (
            &node.name,
            node.date_added.to_string(),
            node.date_modified.to_string(),
            &node.node_category,
        ),
    )?;

    //Node::new("Cake".to_owned(), "event".to_owned())
    //    .create(&TESTING_SQLITE_CONNECTION)
    //    .unwrap();
    Ok(())
}

#[test]
fn test_read_node() {
    Node::new("Cake".to_owned(), "event".to_owned())
        .create(&TESTING_SQLITE_CONNECTION)
        .unwrap();

    let node = Node::read("Cake", &TESTING_SQLITE_CONNECTION).unwrap();

    assert_eq!(node.node_category(), "event");
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[test]
fn test_rusqlite_showcase() -> Result<(), Error> {
    let conn = TESTING_SQLITE_CONNECTION.to_owned().connect().unwrap();

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    let stuff: Vec<Person> = person_iter.map(|x| x.unwrap()).collect();

    stuff.iter().for_each(|x| {
        println!(
            "MEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEP {:?}",
            x
        )
    });

    assert_ne!(stuff.len(), 0);

    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';")
        .unwrap();

    let some_iter = stmt
        .query_map([], |row| Ok(Message(row.get(0).unwrap())))
        .unwrap();

    let stuff: Vec<Message> = some_iter.map(|x| x.unwrap()).collect();

    assert_ne!(stuff.len(), 0);
    Ok(())
}

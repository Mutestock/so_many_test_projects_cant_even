use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{
    misc::time_management::{NaiveDateTimeRusqlite},
    model::model_common::ModelCommon,
};

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub date_added: NaiveDateTime,
    pub date_modified: NaiveDateTime,
    pub primary_image_path: Option<String>,
    pub node_category: String,
}

impl Node {
    pub fn new(name: String, node_category: String) -> Self {
        Self {
            name,
            date_added: NaiveDateTime::now(),
            date_modified: NaiveDateTime::now(),
            primary_image_path: None,
            node_category,
        }
    }

    pub fn as_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
    pub fn node_category(&self) -> &str {
        &self.node_category
    }
}

impl ModelCommon<&str> for Node {
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        let query = "
        CREATE TABLE IF NOT EXISTS Node(
            name TEXT UNIQUE PRIMARY KEY NOT NULL,
            date_added TEXT NOT NULL,
            date_modified TEXT NOT NULL,
            primary_image_path TEXT UNIQUE,
            category_name TEXT NOT NULL,
            FOREIGN KEY(category_name) REFERENCES NodeCategory(category_name)
        )";

        connection.execute(query, ())?;
        Ok(())
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            concat!(
                "INSERT INTO Node (name, date_added, date_modified, category_name)",
                "VALUES (?1, ?2, ?3, ?4);"
            ),
            (
                &self.name,
                self.date_added.to_format(),
                self.date_modified.to_format(),
                &self.node_category,
            ),
        )?;
        Ok(())
    }

    fn read(t: &str, connection: &rusqlite::Connection) -> Result<Node, rusqlite::Error> {
        let mut stmt = connection
            .prepare("SELECT date_added, date_modified, primary_image_path, category_name FROM Node WHERE name = ?1")?;

        let mut some_iter = stmt.query_map([t], |row| {
            Ok(Node {
                name: t.to_owned(),
                date_added: NaiveDateTime::from_row(row, 0),
                date_modified: NaiveDateTime::from_row(row, 1),
                primary_image_path: row.get(2)?,
                node_category: row.get(3)?,
            })
        })?;
        Ok(some_iter.next().unwrap()?)
    }

    fn read_list(connection: &rusqlite::Connection) -> Vec<Node> {
        todo!()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) {
        todo!()
    }

    fn delete(t: &str, connection: &rusqlite::Connection) {
        todo!()
    }
}

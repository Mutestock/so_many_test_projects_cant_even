use chrono::NaiveDateTime;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::{
    commands::command_utils::CommandMessageComposable,
    misc::time_management::NaiveDateTimeExtension, model::model_common::ModelCommon,
};

#[derive(Serialize, Deserialize)]
pub struct Node {
    name: String,
    date_added: NaiveDateTime,
    date_modified: NaiveDateTime,
    primary_image_path: Option<String>,
    node_category: String,
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

    pub fn read_nodes_by_node_category(
        connection: &rusqlite::Connection,
        node_category: &str,
    ) -> Result<Vec<Node>, rusqlite::Error> {
        Self::read_nodes_by_identifier(connection, "category_name", node_category)
    }

    fn read_nodes_by_identifier(
        connection: &rusqlite::Connection,
        identifier: &str,
        identifier_value: &str,
    ) -> Result<Vec<Node>, rusqlite::Error> {
        connection
            .prepare(&format!(
                "
            SELECT name, date_added, date_modified, primary_image_path, category_name
            FROM Node 
            WHERE {}=?1;",
                identifier
            ))?
            .query_map([identifier_value], |row| Node::from_row(None, row))?
            .collect()
    }

    pub fn update_node_category(
        &self,
        new_category: &str,
        connection: &rusqlite::Connection,
    ) -> Result<(), rusqlite::Error> {
        connection
            .prepare(
                "
                UPDATE Node
                SET category_name = ?1
                WHERE name = ?2
            ",
            )?
            .execute(params![new_category, &self.name])?;

        Ok(())
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

    fn create(&self, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
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
        )
    }

    fn read(t: &str, connection: &rusqlite::Connection) -> Result<Option<Node>, rusqlite::Error> {
        let mut nodes = connection
            .prepare(
                "
                SELECT date_added, date_modified, primary_image_path, category_name 
                FROM Node 
                WHERE name = ?1;",
            )?
            .query_map([t], |row| Node::from_row(Some(t), row))?
            .collect::<Vec<Result<Node, rusqlite::Error>>>()
            .into_iter()
            .map(|node_res| Some(node_res.unwrap()))
            .collect::<Vec<Option<Node>>>();

        if nodes.len() == 0 {
            Ok(None)
        } else {
            Ok(nodes.swap_remove(0))
        }
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<Node>, rusqlite::Error> {
        connection
            .prepare(
                "SELECT name, date_added, date_modified, primary_image_path, category_name FROM Node",
            )?
            .query_map([], |row| Node::from_row(None, row))?
            .collect()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        connection
            .prepare(
                "
                UPDATE Node
                SET date_added = ?1,
                    date_modified = ?2,
                    primary_image_path = ?3,
                    category_name = ?4

                WHERE name = ?5;
            ",
            )?
            .execute((
                self.date_added.to_format(),
                NaiveDateTime::now().to_format(),
                &self.primary_image_path,
                &self.node_category,
                t,
            ))
    }

    fn delete(t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        connection
            .prepare("DELETE FROM node WHERE name = ?1")?
            .execute((t,))
    }

    fn from_row(p_key: Option<&str>, row: &rusqlite::Row) -> Result<Node, rusqlite::Error> {
        let p_img_path: Option<String> = row.get(2)?;
        match p_key {
            Some(p_key_val) => Ok(Node {
                name: p_key_val.to_owned(),
                date_added: NaiveDateTime::from_row(row, 0),
                date_modified: NaiveDateTime::from_row(row, 1),
                primary_image_path: p_img_path,
                node_category: row.get(3)?,
            }),
            None => Ok(Node {
                name: row.get(0)?,
                date_added: NaiveDateTime::from_row(row, 1),
                date_modified: NaiveDateTime::from_row(row, 2),
                primary_image_path: p_img_path,
                node_category: row.get(4)?,
            }),
        }
    }
}

impl CommandMessageComposable<Node> for Node {}
impl CommandMessageComposable<Option<Node>> for Option<Node> {}
impl CommandMessageComposable<Vec<Node>> for Vec<Node> {}

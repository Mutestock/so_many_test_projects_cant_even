use std::fmt::Display;

use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::commands::command_utils::CommandMessageComposable;

use super::{model_common::ModelCommon, node::Node};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum NodePreset {
    Event,
    Person,
    Document,
    Location,
    Appointment,
    Custom,
}

impl Display for NodePreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodePreset::Event => write!(f, "Event"),
            NodePreset::Person => write!(f, "Person"),
            NodePreset::Document => write!(f, "Document"),
            NodePreset::Location => write!(f, "Location"),
            NodePreset::Appointment => write!(f, "Appointment"),
            NodePreset::Custom => write!(f, "Custom"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NodeCategory {
    name: String,
}

impl NodeCategory {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl ModelCommon<&str> for NodeCategory {
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS NodeCategory(
                category_name TEXT NOT NULL UNIQUE PRIMARY KEY
            );",
            (),
        )?;
        connection.execute(
            "INSERT OR IGNORE INTO NodeCategory (category_name) 
                VALUES ('event'), ('person'), ('document'), ('location'), ('appointment'), ('none');",
            (),
        )?;
        Ok(())
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        connection.execute(
            "INSERT INTO NodeCategory(category_name) VALUES (?1);",
            (&self.name,),
        )
    }

    fn read(
        t: &str,
        connection: &rusqlite::Connection,
    ) -> Result<Option<NodeCategory>, rusqlite::Error> {
        // This is utterly pointless right now
        let mut node_categories = connection
            .prepare("SELECT category_name FROM NodeCategory WHERE category_name = ?1")?
            .query_map([t], |row| NodeCategory::from_row(Some(t), row))?
            .collect::<Vec<Result<NodeCategory, rusqlite::Error>>>()
            .into_iter()
            .map(|node_category_res| Some(node_category_res.unwrap()))
            .collect::<Vec<Option<NodeCategory>>>();

        if node_categories.len() == 0 {
            Ok(None)
        } else {
            Ok(node_categories.swap_remove(0))
        }
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<NodeCategory>, rusqlite::Error>
    where
        Self: Sized,
    {
        connection
            .prepare("SELECT category_name FROM NodeCategory")?
            .query_map([], |row| NodeCategory::from_row(None, row))?
            .collect()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        Node::read_list(connection)?
            .iter()
            .for_each(|node| node.update_node_category(t, connection).unwrap());
        connection
            .prepare("UPDATE NodeCategory SET category_name = ?1 WHERE category_name = ?2")?
            .execute(params![self.name, t])
    }

    fn delete(t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        Node::read_list(connection)?
            .iter()
            .for_each(|node| node.update_node_category("none", connection).unwrap());
        connection
            .prepare(
                "
            DELETE FROM NodeCategory WHERE category_name = ?1;
            ",
            )?
            .execute(params![t])
    }

    fn from_row(p_key: Option<&str>, row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        match p_key {
            Some(val) => Ok(NodeCategory {
                name: val.to_owned(),
            }),
            None => Ok(NodeCategory { name: row.get(0)? }),
        }
    }
}

impl CommandMessageComposable<NodeCategory> for NodeCategory {}
impl CommandMessageComposable<Option<NodeCategory>> for Option<NodeCategory> {}
impl CommandMessageComposable<Vec<NodeCategory>> for Vec<NodeCategory> {}
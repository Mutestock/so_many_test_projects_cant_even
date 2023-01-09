use regex::Regex;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use super::{model_common::ModelCommon, node::Node};
use crate::commands::command_utils::CommandResponseComposable;

lazy_static::lazy_static! {
    pub static ref DEFAULT_CATEGORIES: [NodeCategory; 8] = [
        NodeCategory::new("event".to_owned(), "#3C92E8".to_owned()),
        NodeCategory::new("person".to_owned(), "#4A3CE8".to_owned()),
        NodeCategory::new("document".to_owned(), "#1DADA6".to_owned()),
        NodeCategory::new("location".to_owned(), "#BF5217".to_owned()),
        NodeCategory::new("appointment".to_owned(), "#C7BC77".to_owned()),
        NodeCategory::new("bill".to_owned(), "#A88D20".to_owned()),
        NodeCategory::new("warranty".to_owned(), "#A1571A".to_owned()),
        NodeCategory::new("none".to_owned(), "#74A37D".to_owned()),

    ];

}

#[derive(Serialize, Deserialize, Clone)]
pub struct NodeCategory {
    name: String,
    color_code_hex: String,
    // Storing this value here persists these preferences in contrast to state.
    // It also allows easier sorting of nodes
    // Although there's an argument to be made for this value being managed by state instead.
    pub visibility_toggled_on: bool,
}

impl NodeCategory {
    pub fn new(name: String, color_code_hex: String) -> Self {
        Self {
            name,
            color_code_hex,
            visibility_toggled_on: true,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_valid_hex(color_code_hex: &str) -> bool {
        Regex::new("^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$")
            .unwrap()
            .is_match(color_code_hex)
    }

    pub fn update_category_toggle_visisbility(
        category: &str,
        connection: &rusqlite::Connection,
    ) -> Result<usize, rusqlite::Error> {
        connection
            .prepare(
                "
        UPDATE NodeCategory
        SET visibility_toggled_on = CASE
            WHEN visibility_toggled_on = 1
                THEN 0
            ELSE 1
            END
        WHERE category_name = ?1;
        ",
            )?
            .execute(params![category])
    }
}

impl ModelCommon<&str> for NodeCategory {
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS NodeCategory(
                category_name TEXT NOT NULL UNIQUE PRIMARY KEY,
                color_code_hex TEXT NOT NULL,
                visibility_toggled_on BOOLEAN DEFAULT 1
            );",
            (),
        )?;
        let mut query = String::from(
            "INSERT OR IGNORE INTO NodeCategory (category_name, color_code_hex) VALUES",
        );

        DEFAULT_CATEGORIES.iter().for_each(|x| {
            query = format!("{} ('{}', '{}')", query, x.name, x.color_code_hex);
            if DEFAULT_CATEGORIES.last().unwrap().name != x.name {
                query = format!("{},", query);
            }
        });
        query = format!("{};", query);

        connection.execute(&query, ())?;
        Ok(())
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        if !NodeCategory::is_valid_hex(&self.color_code_hex) {
            return Ok(0);
        }

        connection.execute(
            "INSERT INTO NodeCategory(category_name, color_code_hex) VALUES (?1, ?2);",
            (&self.name, &self.color_code_hex),
        )
    }

    fn read(
        t: &str,
        connection: &rusqlite::Connection,
    ) -> Result<Option<NodeCategory>, rusqlite::Error> {
        let mut node_categories = connection
            .prepare(
                "SELECT category_name, color_code_hex, visibility_toggled_on FROM NodeCategory WHERE category_name = ?1;",
            )?
            .query_map([t], |row| NodeCategory::from_row(None, row))?
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
            .prepare(
                "SELECT category_name, color_code_hex, visibility_toggled_on FROM NodeCategory;",
            )?
            .query_map([], |row| NodeCategory::from_row(None, row))?
            .collect()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        if !NodeCategory::is_valid_hex(&self.color_code_hex) {
            return Ok(0);
        }

        Node::read_list(connection)?
            .iter()
            .for_each(|node| node.update_node_category(t, connection).unwrap());
        connection
            .prepare(
                "
                UPDATE NodeCategory 
                SET category_name = ?1, 
                    color_code_hex = ?2,
                    visibility_toggled_on = ?3
                
                WHERE category_name = ?4
                ",
            )?
            .execute(params![
                self.name,
                self.color_code_hex,
                self.visibility_toggled_on,
                t
            ])
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
                color_code_hex: row.get(0)?,
                visibility_toggled_on: row.get(1)?,
            }),
            None => Ok(NodeCategory {
                name: row.get(0)?,
                color_code_hex: row.get(1)?,
                visibility_toggled_on: row.get(2)?,
            }),
        }
    }
}

impl CommandResponseComposable<NodeCategory> for NodeCategory {}
impl CommandResponseComposable<Option<NodeCategory>> for Option<NodeCategory> {}
impl CommandResponseComposable<Vec<NodeCategory>> for Vec<NodeCategory> {}

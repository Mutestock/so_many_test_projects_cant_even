use serde::{Deserialize, Serialize};

use crate::commands::command_utils::CommandResponseComposable;

#[derive(Serialize, Deserialize)]
pub struct NodeVisual {
    node_name: String,
    color_code_hex: String,
}

impl NodeVisual {
    pub fn new(node_name: String, color_code_hex: String) -> Self {
        Self {
            node_name,
            color_code_hex,
        }
    }

    pub fn read_list_toggled_on(
        connection: &rusqlite::Connection,
    ) -> Result<Vec<NodeVisual>, rusqlite::Error> {
        connection
            .prepare(
                "
            SELECT n.name, c.color_code_hex 
            FROM Node n 
            INNER JOIN Category c
            ON n.category_name = c.category_name
            WHERE c.visibility_toggled_on;
        ",
            )?
            .query_map([], |row| Ok(NodeVisual::new(row.get(0)?, row.get(1)?)))?
            .collect()
    }
}

impl CommandResponseComposable<Vec<NodeVisual>> for Vec<NodeVisual> {}

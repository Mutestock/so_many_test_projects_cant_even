use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::commands::command_utils::CommandResponseComposable;

#[derive(Serialize, Deserialize)]
pub struct NodeTag {
    tag_name: String,
    node_name: String,
}

impl NodeTag {
    pub fn new(tag_name: String, node_name: String) -> Self {
        Self {
            tag_name,
            node_name,
        }
    }

    pub fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection
            .prepare(
                "        
                CREATE TABLE IF NOT EXISTS Node_Tag(
                    tag_name TEXT REFERENCES Tag(name),
                    node_name TEXT REFERENCES Node(name)
                );",
            )?
            .execute(())?;
        Ok(())
    }

    pub fn delete_by_node_name(
        node_name: &str,
        connection: &rusqlite::Connection,
    ) -> Result<usize, rusqlite::Error> {
        connection
            .prepare("DELETE FROM NodeTag where node_name = ?1")?
            .execute(params![node_name])
    }

    pub fn delete_by_tag_name(
        tag_name: &str,
        connection: &rusqlite::Connection,
    ) -> Result<usize, rusqlite::Error> {
        connection
            .prepare("DELETE FROM NodeTag where tag_name = ?1")?
            .execute(params![tag_name])
    }
}

impl CommandResponseComposable<NodeTag> for NodeTag {}
impl CommandResponseComposable<Option<NodeTag>> for Option<NodeTag> {}
impl CommandResponseComposable<Vec<NodeTag>> for Vec<NodeTag> {}

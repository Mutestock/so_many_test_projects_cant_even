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
}

impl CommandResponseComposable<NodeTag> for NodeTag {}
impl CommandResponseComposable<Option<NodeTag>> for Option<NodeTag> {}
impl CommandResponseComposable<Vec<NodeTag>> for Vec<NodeTag> {}

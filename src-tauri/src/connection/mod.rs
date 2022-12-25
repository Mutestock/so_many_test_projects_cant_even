pub mod sqlite_connection;
pub mod connection_common;

use crate::model::{node::Node, node_category::NodeCategory, node_comment::NodeComment, model_common::ModelCommon};

use sqlite_connection::SqliteConnection;

pub fn initialize(connection: &SqliteConnection) -> Result<(), rusqlite::Error> {
    NodeCategory::init_script(connection)?;
    Node::init_script(connection)?;
    NodeComment::init_script(connection)?;
    Ok(())
}

pub mod connection_common;
pub mod sqlite_connection;

use crate::model::{
    model_common::ModelCommon, node::Node, node_category::NodeCategory, node_comment::NodeComment, node_image::NodeImage,
};

pub fn initialize(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    // Order matters here
    NodeCategory::init_script(connection)?;
    Node::init_script(connection)?;
    NodeComment::init_script(connection)?;
    NodeImage::init_script(connection)?;
    Ok(())
}

pub mod connection_common;
pub mod sqlite_connection;

use crate::model::{
    category::Category, comment::Comment, image::Image, model_common::ModelCommon, node::Node,
    node_tag::NodeTag, tag::Tag,
};

pub fn initialize(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    // Order matters here
    Category::init_script(connection)?;
    Node::init_script(connection)?;
    Comment::init_script(connection)?;
    Image::init_script(connection)?;
    Tag::init_script(connection)?;
    NodeTag::init_script(connection)?;
    Ok(())
}

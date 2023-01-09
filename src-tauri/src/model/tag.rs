use serde::{Serialize, Deserialize};

use crate::model::model_common::ModelCommon;
use crate::commands::command_utils::CommandResponseComposable;


#[derive(Serialize, Deserialize)]
pub struct Tag{
    name: String
}


impl ModelCommon<&str> for Tag{
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.prepare("
        CREATE TABLE IF NOT EXISTS Tag (
            name TEXT PRIMARY KEY NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS Node_Tag(
            tag_name TEXT REFERENCES Tag(name),
            node_name TEXT REFERENCES Node(name)
        );
        ")?
        .execute(())?;

        Ok(())
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        todo!()
    }

    fn read(t: &str, connection: &rusqlite::Connection) -> Result<Option<Self>, rusqlite::Error>
    where
        Self: Sized {
        todo!()
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<Self>, rusqlite::Error>
    where
        Self: Sized {
        todo!()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        todo!()
    }

    fn delete(t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        todo!()
    }

    fn from_row(p_key: Option<&str>, row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized {
        todo!()
    }
}


impl CommandResponseComposable<Tag> for Tag {}
impl CommandResponseComposable<Option<Tag>> for Option<Tag> {}
impl CommandResponseComposable<Vec<Tag>> for Vec<Tag> {}
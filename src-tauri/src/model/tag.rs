use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::commands::command_utils::CommandResponseComposable;
use crate::model::model_common::ModelCommon;

#[derive(Serialize, Deserialize)]
pub struct Tag {
    name: String,
}

impl ModelCommon<&str> for Tag {
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection
            .prepare(
                "
        CREATE TABLE IF NOT EXISTS Tag (
            name TEXT PRIMARY KEY NOT NULL UNIQUE
        );
        ",
            )?
            .execute(())?;

        Ok(())
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        connection
            .prepare(
                "
            INSERT INTO tag (name) VALUES (?1);
        ",
            )?
            .execute(params![&self.name])
    }

    // Not currently compatible
    fn read(_t: &str, _connection: &rusqlite::Connection) -> Result<Option<Self>, rusqlite::Error>
    where
        Self: Sized,
    {
        Ok(None)
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<Self>, rusqlite::Error>
    where
        Self: Sized,
    {
        connection
            .prepare("SELECT name FROM Tag")?
            .query_map([], |row| Tag::from_row(None, row))?
            .collect()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        connection
            .prepare("UPDATE NodeTag SET tag_name = ?1 WHERE tag_name = ?2")?
            .execute(params![self.name, t])?;
        connection
            .prepare("UPDATE Tag SET name = ?1 WHERE name = ?2")?
            .execute(params![self.name, t])
    }

    fn delete(t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        connection
            .prepare("DELETE FROM NodeTag where tag_name = ?1")?
            .execute(params![t])?;
        connection
            .prepare("DELETE FROM tag WHERE name = ?1")?
            .execute(params![t])
    }

    fn from_row(p_key: Option<&str>, row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        match p_key {
            Some(name) => Ok(Tag {
                name: name.to_owned(),
            }),
            None => Ok(Tag { name: row.get(0)? }),
        }
    }
}

impl CommandResponseComposable<Tag> for Tag {}
impl CommandResponseComposable<Option<Tag>> for Option<Tag> {}
impl CommandResponseComposable<Vec<Tag>> for Vec<Tag> {}

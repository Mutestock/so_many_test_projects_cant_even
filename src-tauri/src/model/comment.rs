use crate::{
    commands::command_utils::CommandResponseComposable,
    misc::time_management::NaiveDateTimeExtension,
};

use super::model_common::ModelCommon;
use chrono::NaiveDateTime;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Comment {
    uuid: String,
    node_name: String,
    content: String,
    date_added: NaiveDateTime,
    date_modified: NaiveDateTime,
}

impl Comment {
    pub fn new(node_name: String, content: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            node_name,
            content,
            date_added: NaiveDateTime::now(),
            date_modified: NaiveDateTime::now(),
        }
    }
    pub fn uuid(&self) -> &str {
        &self.uuid
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn delete_by_node_name(
        connection: &rusqlite::Connection,
        node_name: &str,
    ) -> Result<usize, rusqlite::Error> {
        connection
            .prepare("DELETE FROM Comment WHERE node_name = ?1;")?
            .execute(params![node_name])
    }

    pub fn update_comment_content_by_node_name(
        connection: &rusqlite::Connection,
        old_node_name: &str,
        new_comment_content: &str,
    ) -> Result<usize, rusqlite::Error> {
        connection
            .prepare(
                "
                UPDATE Comment
                SET content = ?1
                WHERE node_name = ?2;
            ",
            )?
            .execute(params![new_comment_content, old_node_name])
    }

    fn read_by_identifier(
        connection: &rusqlite::Connection,
        identifier: &str,
        identifier_value: &str,
    ) -> Result<Option<Comment>, rusqlite::Error> {
        let mut comments = connection
            .prepare(&format!(
                "
                SELECT uuid, node_name, content, date_added, date_modified 
                FROM Comment 
                WHERE {}=?1;",
                identifier
            ))?
            .query_map([identifier_value], |row| Comment::from_row(None, row))?
            .collect::<Vec<Result<Comment, rusqlite::Error>>>()
            .into_iter()
            .map(|node_res| Some(node_res.unwrap()))
            .collect::<Vec<Option<Comment>>>();

        if comments.len() == 0 {
            Ok(None)
        } else {
            Ok(comments.swap_remove(0))
        }
    }

    pub fn read_comment_by_node_name(
        connection: &rusqlite::Connection,
        node_name: &str,
    ) -> Result<Option<Comment>, rusqlite::Error> {
        Self::read_by_identifier(connection, "node_name", node_name)
    }
}

impl ModelCommon<&str> for Comment {
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            concat!(
                "CREATE TABLE IF NOT EXISTS Comment (",
                "    uuid TEXT PRIMARY KEY NOT NULL UNIQUE,",
                "    content TEXT,",
                "    date_added TEXT NOT NULL,",
                "    date_modified TEXT,",
                "    node_name TEXT NOT NULL,",
                "    FOREIGN KEY(node_name) REFERENCES Node(name)",
                ");"
            ),
            (),
        )?;
        Ok(())
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        connection.execute(
            concat!(
                "INSERT INTO Comment(uuid, content, date_added, date_modified, node_name)",
                "VALUES( ?1, ?2, ?3, ?4, ?5)"
            ),
            (
                &self.uuid,
                &self.content,
                self.date_added.to_format(),
                self.date_modified.to_format(),
                &self.node_name,
            ),
        )
    }

    fn read(
        t: &str,
        connection: &rusqlite::Connection,
    ) -> Result<Option<Comment>, rusqlite::Error> {
        Self::read_by_identifier(connection, "uuid", t)
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<Comment>, rusqlite::Error>
    where
        Self: Sized,
    {
        connection
            .prepare("SELECT uuid, node_name, content, date_added, date_modified FROM Comment")?
            .query_map([], |row| Comment::from_row(None, row))?
            .collect()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        connection
            .prepare(
                "
                UPDATE Comment
                SET node_name = ?1,
                    content = ?2,
                    date_added = ?3,
                    date_modified = ?4
                
                WHERE uuid = ?5
            ",
            )?
            .execute(params![
                &self.node_name,
                &self.content,
                self.date_added.to_format(),
                NaiveDateTime::now().to_format(),
                t
            ])
    }

    fn delete(t: &str, connection: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        connection
            .prepare(
                "
                DELETE FROM Comment 
                WHERE uuid=?1",
            )?
            .execute(params![t])
    }

    fn from_row(p_key: Option<&str>, row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        match p_key {
            Some(val) => Ok(Comment {
                uuid: val.to_owned(),
                node_name: row.get(0)?,
                content: row.get(1)?,
                date_added: NaiveDateTime::from_row(row, 2),
                date_modified: NaiveDateTime::from_row(row, 3),
            }),
            None => Ok(Comment {
                uuid: row.get(0)?,
                node_name: row.get(1)?,
                content: row.get(2)?,
                date_added: NaiveDateTime::from_row(row, 3),
                date_modified: NaiveDateTime::from_row(row, 4),
            }),
        }
    }
}

impl CommandResponseComposable<Comment> for Comment {}
impl CommandResponseComposable<Option<Comment>> for Option<Comment> {}
impl CommandResponseComposable<Vec<Comment>> for Vec<Comment> {}

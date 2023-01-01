use crate::misc::time_management::NaiveDateTimeExtension;

use super::model_common::ModelCommon;
use chrono::NaiveDateTime;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct NodeComment {
    uuid: String,
    node_name: String,
    content: String,
    date_added: NaiveDateTime,
    date_modified: NaiveDateTime,
}

impl NodeComment {
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
    ) -> Result<(), rusqlite::Error> {
        connection
            .prepare("DELETE FROM NodeComment WHERE node_name = ?1;")?
            .execute(params![node_name])?;
        Ok(())
    }

    pub fn update_node_comment_content_by_node_name(
        connection: &rusqlite::Connection,
        old_node_name: &str,
        new_comment_content: &str,
    ) -> Result<bool, rusqlite::Error> {
        connection
            .prepare(
                "
                UPDATE NodeComment
                SET content = ?1
                WHERE node_name = ?2;
            ",
            )?
            .execute(params![new_comment_content, old_node_name])?;

        Ok(true)
    }

    fn read_by_identifier(
        connection: &rusqlite::Connection,
        identifier: &str,
        identifier_value: &str,
    ) -> Result<Option<NodeComment>, rusqlite::Error> {
        let mut node_comments = connection
            .prepare(&format!(
                "
                SELECT uuid, node_name, content, date_added, date_modified 
                FROM NodeComment 
                WHERE {}=?1;",
                identifier
            ))?
            .query_map([identifier_value], |row| NodeComment::from_row(None, row))?
            .collect::<Vec<Result<NodeComment, rusqlite::Error>>>()
            .into_iter()
            .map(|node_res| Some(node_res.unwrap()))
            .collect::<Vec<Option<NodeComment>>>();

        if node_comments.len() == 0 {
            Ok(None)
        } else {
            Ok(node_comments.swap_remove(0))
        }
    }

    pub fn read_node_comment_by_node_name(
        connection: &rusqlite::Connection,
        node_name: &str,
    ) -> Result<Option<NodeComment>, rusqlite::Error> {
        Self::read_by_identifier(connection, "node_name", node_name)
    }
}

impl ModelCommon<&str> for NodeComment {
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            concat!(
                "CREATE TABLE IF NOT EXISTS NodeComment (",
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

    fn create(&self, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection.execute(
            concat!(
                "INSERT INTO NodeComment(uuid, content, date_added, date_modified, node_name)",
                "VALUES( ?1, ?2, ?3, ?4, ?5)"
            ),
            (
                &self.uuid,
                &self.content,
                self.date_added.to_format(),
                self.date_modified.to_format(),
                &self.node_name,
            ),
        )?;

        Ok(())
    }

    fn read(
        t: &str,
        connection: &rusqlite::Connection,
    ) -> Result<Option<NodeComment>, rusqlite::Error> {
        Self::read_by_identifier(connection, "uuid", t)
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<NodeComment>, rusqlite::Error>
    where
        Self: Sized,
    {
        connection
            .prepare("SELECT uuid, node_name, content, date_added, date_modified FROM NodeComment")?
            .query_map([], |row| NodeComment::from_row(None, row))?
            .collect()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection
            .prepare(
                "
                UPDATE NodeComment
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
            ])?;
        Ok(())
    }

    fn delete(t: &str, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection
            .prepare(
                "
                DELETE FROM NodeComment 
                WHERE uuid=?1",
            )?
            .execute(params![t])?;

        Ok(())
    }

    fn from_row(p_key: Option<&str>, row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        match p_key {
            Some(val) => Ok(NodeComment {
                uuid: val.to_owned(),
                node_name: row.get(0)?,
                content: row.get(1)?,
                date_added: NaiveDateTime::from_row(row, 2),
                date_modified: NaiveDateTime::from_row(row, 3),
            }),
            None => Ok(NodeComment {
                uuid: row.get(0)?,
                node_name: row.get(1)?,
                content: row.get(2)?,
                date_added: NaiveDateTime::from_row(row, 3),
                date_modified: NaiveDateTime::from_row(row, 4),
            }),
        }
    }
}

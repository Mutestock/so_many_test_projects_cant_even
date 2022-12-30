use std::{path::PathBuf, str::FromStr};

use chrono::NaiveDateTime;
use rusqlite::params;

use crate::misc::{directories::BASE_IMAGE_PATH, time_management::NaiveDateTimeExtension};

use super::model_common::ModelCommon;

pub struct NodeImage {
    image_title: String,
    image_path: PathBuf,
    date_added: NaiveDateTime,
    date_modified: NaiveDateTime,
    node_name: String,
}

impl NodeImage {
    pub fn new(image_title: String, node_name: String) -> Self {
        Self {
            image_title: image_title.clone(),
            image_path: BASE_IMAGE_PATH.join(image_title),
            date_added: NaiveDateTime::now(),
            date_modified: NaiveDateTime::now(),
            node_name,
        }
    }
    pub fn node_name(&self) -> &str {
        &self.node_name
    }
    pub fn image_title(&self) -> &str {
        &self.image_title
    }
    pub fn delete_by_node_name(
        connection: &rusqlite::Connection,
        node_name: &str,
    ) -> Result<(), rusqlite::Error> {
        connection
            .prepare(" DELETE FROM NodeImage WHERE node_name = ?1;")?
            .execute(params![node_name])?;
        Ok(())
    }
    pub fn update_by_node_name(
        connection: &rusqlite::Connection,
        old_node_name: &str,
        new_node_name: &str,
    ) -> Result<(), rusqlite::Error> {
        connection
            .prepare("UPDATE NodeImage SET node_name = ?1 WHERE node_name = ?2;")?
            .execute(params![old_node_name, new_node_name])?;
        Ok(())
    }
}

impl ModelCommon<&str> for NodeImage {
    fn init_script(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection
            .prepare(
                "
                CREATE TABLE IF NOT EXISTS NodeImage (
                    image_title TEXT UNIQUE PRIMARY KEY NOT NULL,
                    image_path TEXT NOT NULL,
                    date_added TEXT NOT NULL,
                    date_modified TEXT NOT NULL,
                    node_name TEXT NOT NULL,
                    FOREIGN KEY(node_name) REFERENCES Node(name)
                );",
            )?
            .execute(())?;
        Ok(())
    }

    fn create(&self, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection
            .prepare(
                "
            INSERT INTO NodeImage (image_title, image_path, date_added, date_modified, node_name)
            VALUES ( ?1, ?2, ?3, ?4, ?5);",
            )?
            .execute(params![
                &self.image_title,
                self.image_path.to_str(),
                self.date_added.to_format(),
                self.date_modified.to_format(),
                &self.node_name
            ])?;
        Ok(())
    }

    fn read(t: &str, connection: &rusqlite::Connection) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        connection
            .prepare(
                "
                SELECT image_path, date_added, date_modified, node_name
                FROM NodeImage
                WHERE image_title = ?1;
            ",
            )?
            .query_map(params![t], |row| NodeImage::from_row(Some(t), row))?
            .next()
            .unwrap()
    }

    fn read_list(connection: &rusqlite::Connection) -> Result<Vec<NodeImage>, rusqlite::Error>
    where
        Self: Sized,
    {
        connection
            .prepare(
                "
            SELECT image_title, image_path, date_added, date_modified, node_name
            FROM NodeImage
        ",
            )?
            .query_map(params![], |row| NodeImage::from_row(None, row))?
            .collect()
    }

    fn update(&self, t: &str, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection
            .prepare(
                "
            UPDATE NodeImage
            SET image_title = ?1,
                image_path = ?2,
                date_added = ?3,
                date_modified = ?4,
                node_name = ?5
            
            WHERE image_title = ?6
        ",
            )?
            .execute(params![
                &self.image_title,
                self.image_path.to_str(),
                self.date_added.to_format(),
                NaiveDateTime::now().to_format(),
                &self.node_name,
                t
            ])?;
        Ok(())
    }

    fn delete(t: &str, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
        connection
            .prepare(
                "
            DELETE FROM NodeImage 
            WHERE image_title = ?1;",
            )?
            .execute(params![t])?;
        Ok(())
    }

    fn from_row(p_key: Option<&str>, row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        match p_key {
            Some(val) => Ok(NodeImage {
                image_title: val.to_owned(),
                image_path: {
                    let img_path_as_string: String = row.get(0)?;
                    PathBuf::from_str(&img_path_as_string).unwrap()
                },
                date_added: NaiveDateTime::from_row(row, 1),
                date_modified: NaiveDateTime::from_row(row, 2),
                node_name: row.get(3)?,
            }),
            None => Ok(NodeImage {
                image_title: row.get(0)?,
                image_path: {
                    let img_path_as_string: String = row.get(1)?;
                    PathBuf::from_str(&img_path_as_string).unwrap()
                },
                date_added: NaiveDateTime::from_row(row, 2),
                date_modified: NaiveDateTime::from_row(row, 3),
                node_name: row.get(4)?,
            }),
        }
    }
}

use std::fmt::Display;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{
    connection::connection_common::MindmapConnector,
    errors::{CustomNodeWithoutNameError},
};
use crate::connection::sqlite_connection::SqliteConnection;
use crate::model::{node_comment::NodeComment, model_common::ModelCommon};


#[derive(Serialize, Deserialize)]
pub struct Node {
    name: String,
    comments: Vec<NodeComment>,
    date_added: DateTime<Local>,
    date_modified: DateTime<Local>,
    image_paths: Option<Vec<String>>,
    primary_image_path: Option<String>,
    node_category: String,
}

impl Node {
    pub fn new(
        node_type: usize,
        name: String,
        node_category: String,
    ) -> Result<Self, CustomNodeWithoutNameError> {
        Ok(Self {
            name,
            comments: vec![],
            date_added: Local::now(),
            date_modified: Local::now(),
            image_paths: None,
            primary_image_path: None,
            node_category: node_category,
        })
    }

    pub fn as_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn and_add_comment(mut self, node_comment: NodeComment) -> Self {
        self.comments.push(node_comment);
        self
    }
}

impl ModelCommon<&str> for Node {
    fn init_script(&self, connector: SqliteConnection) {
        connector
            .connect()
            .expect("Could not create sqlite connection for Node init script")
            .execute(
                concat!(
                    "CREATE TABLE IF NOT EXISTS Node(", 
                    "   id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,",
                    "   name TEXT UNIQUE NOT NULL,",
                    "   date_added TEXT NOT NULL,",
                    "   date_modified TEXT NOT NULL,",
                    "   id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,",
                    "   primary_image_path TEXT UNIQUE,",
                    "   category_name TEXT NOT NULL,",
                    "   FOREIGN KEY(category_name) REFERENCES NodeCategory",
                    ");",
                ), ()
            );
    }

    fn create(&self, connector: SqliteConnection) {
        todo!()
    }

    fn read(t: &str, connector: SqliteConnection) -> Node {
        todo!()
    }

    fn read_list(connector: SqliteConnection) -> Vec<Node> {
        todo!()
    }

    fn update(&self, t: &str, connector: SqliteConnection) {
        todo!()
    }

    fn delete(t: &str, connector: SqliteConnection) {
        todo!()
    }

    fn get_init_order(&self) -> u8 {
        todo!()
    }
}

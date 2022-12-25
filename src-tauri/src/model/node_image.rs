use crate::connection::sqlite_connection::SqliteConnection;

use super::model_common::ModelCommon;

pub struct NodeImage{
    image_title: String,
    image_path: Option<String>,
    node_name: String,
}

impl NodeImage{
    pub fn new(image_title: String, node_name: String) -> Self{
        Self {
            image_title,
            image_path: None,
            node_name
        }
    }
}

impl ModelCommon<&str> for NodeImage{
    fn init_script( connector: &SqliteConnection ) -> Result<(), rusqlite::Error> {
        todo!()
    }

    fn create(&self, connector: &SqliteConnection) -> Result<(), rusqlite::Error> {
        todo!()
    }

    fn read(t: &str, connector: &SqliteConnection) -> Result<Self, rusqlite::Error> where Self: Sized {
        todo!()
    }

    fn read_list(connector: &SqliteConnection) -> Vec<Self> where Self: Sized {
        todo!()
    }

    fn update(&self, t: &str, connector: &SqliteConnection) {
        todo!()
    }

    fn delete(t: &str, connector: &SqliteConnection) {
        todo!()
    }
}
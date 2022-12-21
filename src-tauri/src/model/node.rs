
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::errors::{TryFromError, CustomNodeWithoutNameError};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum NodePreset {
    Event,
    Person,
    Document,
    Location,
    Appointment,
    Custom,
}


impl TryFrom<usize> for NodePreset {
    type Error = TryFromError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            value if value == Self::Event as usize => Ok(Self::Event),
            value if value == Self::Person as usize => Ok(Self::Person),
            value if value == Self::Document as usize => Ok(Self::Document),
            value if value == Self::Location as usize => Ok(Self::Location),
            value if value == Self::Appointment as usize => Ok(Self::Appointment),
            value if value == Self::Custom as usize => Ok(Self::Custom),
            _ => Err(TryFromError(
                "Could not cast usize to NodePreset".to_owned(),
            )),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    node_type: NodePreset,
    name: String,
    comments: Vec<NodeComment>,
    date_added: DateTime<Local>,
    date_modified: DateTime<Local>,
    image_paths: Option<Vec<String>>,
    primary_image_path: Option<String>,
    custom_node_type_name: Option<String>,
}

impl Node {
    pub fn new(
        node_type: usize,
        name: String,
        custom_node_name: Option<String>,
    ) -> Result<Self, CustomNodeWithoutNameError> {
        let node_preset =
            NodePreset::try_from(node_type).expect("Node preset failed to be set on new node");
        if node_preset == NodePreset::Custom && custom_node_name == None {
            return Err(CustomNodeWithoutNameError(
                "Can't instantiate a new node of type 'Custom' without a custom node name"
                    .to_owned(),
            ));
        }
        Ok(Self {
            node_type: node_preset,
            name,
            comments: vec![],
            date_added: Local::now(),
            date_modified: Local::now(),
            image_paths: None,
            primary_image_path: None,
            custom_node_type_name: custom_node_name,
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
#[derive(Serialize, Deserialize)]
pub struct NodeComment {
    content: String,
    date_added: DateTime<Local>,
}

impl NodeComment {
    pub fn new(content: String) -> Self {
        Self {
            content,
            date_added: Local::now(),
        }
    }
}

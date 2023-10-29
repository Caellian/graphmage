use crate::{node::NodeId, property::Properties};

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeData {
    pub weight: f32,
    pub properties: Properties,
}

impl Default for EdgeData {
    fn default() -> Self {
        Self {
            weight: 1.0,
            properties: Properties::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub start: Attachment,
    pub end: Attachment,
    pub data: EdgeData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attachment {
    pub node: NodeId,
    pub port: Option<usize>,
}

use generational_arena::Index;
use glam::Vec2;

use crate::property::Properties;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub position: Vec2,
    pub ports: Option<Vec<Vec2>>,
    pub properties: Properties,
}

impl Node {
    pub fn new(position: Vec2) -> Self {
        Node {
            position,
            ports: None,
            properties: Properties::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub(crate) Index);

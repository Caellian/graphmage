use edge::{Attachment, Edge, EdgeData};
use generational_arena::Arena;
use node::{Node, NodeId};
use property::Properties;

pub mod edge;
pub mod node;
pub mod property;

#[derive(Debug, Default)]
pub struct Graph {
    nodes: Arena<Node>,
    edges: Vec<Edge>,

    groups: Vec<Graph>,

    properties: Properties,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Arena::new(),
            edges: Vec::new(),
            groups: Vec::new(),
            properties: Properties::new(),
        }
    }

    pub fn push_node(&mut self, node: Node) -> NodeId {
        NodeId(self.nodes.insert(node))
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id.0)
    }

    pub fn push_edge(&mut self, start: Attachment, end: Attachment, data: EdgeData) {
        self.edges.push(Edge { start, end, data })
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter().map(|it| it.1)
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }
}

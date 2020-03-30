use crate::{
    connection::Connection
};

type Index<NodeData, ConnectionData> = crate::index::Index<Connection<NodeData, ConnectionData>>;

#[derive(Clone)]
pub struct Node<NodeData, ConnectionData> {
    data: NodeData,

    connections: Vec<Index<NodeData, ConnectionData>>
}

impl<NodeData, ConnectionData> Node<NodeData, ConnectionData> {
    pub(crate) fn new(data: NodeData) -> Self {
        Node {
            data,
            connections: Vec::new()
        }
    }

    pub fn data(&self) -> &NodeData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut NodeData {
        &mut self.data
    }

    pub fn connections(&self) -> &[Index<NodeData, ConnectionData>] {
        &self.connections
    }

    pub(crate) fn add_connection(&mut self, idx: Index<NodeData, ConnectionData>) {
        self.connections.push(idx);
    }
}

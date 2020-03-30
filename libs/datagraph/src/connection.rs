use crate::{
    node::Node,
    index::Index
};

type NodeIndex<NodeData, ConnectionData> = Index<Node<NodeData, ConnectionData>>;

#[derive(Clone)]
pub struct Connection<NodeData, ConnectionData> {
    data: ConnectionData,

    nodes: [NodeIndex<NodeData, ConnectionData>; 2]
}

impl<NodeData, ConnectionData> Connection<NodeData, ConnectionData> {
    pub(crate) fn new(from: NodeIndex<NodeData, ConnectionData>, to: NodeIndex<NodeData, ConnectionData>, data: ConnectionData) -> Self {
        Connection {
            data,

            nodes: [from, to]
        }
    }

    pub fn data(&self) -> &ConnectionData {
        &self.data
    }

    pub fn mut_data(&mut self) -> &mut ConnectionData {
        &mut self.data
    }

    pub fn nodes(&self) -> &[NodeIndex<NodeData, ConnectionData>] {
        &self.nodes
    }
}

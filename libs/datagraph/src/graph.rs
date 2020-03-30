use crate::{
    map::IndexMap,
    connection::Connection,
    node::Node,
    index::{Index, Indexable}
};

pub struct Graph<NodeData, ConnectionData> {
    pub(crate) nodes: IndexMap<Node<NodeData, ConnectionData>>,
    pub(crate) connections: IndexMap<Connection<NodeData, ConnectionData>>,
}

impl<NodeData, ConnectionData> Graph<NodeData, ConnectionData> {
    pub fn new() -> Self {
        Graph {
            nodes: IndexMap::new(),
            connections: IndexMap::new()
        }
    }

    pub fn add_node(&mut self, data: NodeData) -> Index<Node<NodeData, ConnectionData>> {
        self.nodes.insert(Node::new(data))
    }

    pub fn connect_nodes(&mut self, from: Index<Node<NodeData, ConnectionData>>, to: Index<Node<NodeData, ConnectionData>>, data: ConnectionData) -> Index<Connection<NodeData, ConnectionData>> {
        let idx = self.connections.insert(Connection::new(from, to, data));
        self.get_mut(from).unwrap().add_connection(idx);
        self.get_mut(to).unwrap().add_connection(idx);
        idx
    }

    pub fn get<T: Indexable<NodeData, ConnectionData>>(&self, idx: Index<T>) -> Option<&T> {
        T::get(self, idx)
    }

    pub fn get_mut<T: Indexable<NodeData, ConnectionData>>(&mut self, idx: Index<T>) -> Option<&mut T> {
        T::get_mut(self, idx)
    }
}

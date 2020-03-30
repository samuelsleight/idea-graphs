use std::{
    marker::PhantomData
};

use crate::{
    graph::Graph,
    node::Node,
    connection::Connection
};

pub struct Index<T>(pub(crate) usize, PhantomData<T>);

impl<T> Clone for Index<T> {
    fn clone(&self) -> Index<T> {
        Self(self.0, self.1)
    }
}

impl<T> Copy for Index<T> {}

impl<T> PartialEq for Index<T> {
    fn eq(&self, other: &Index<T>) -> bool {
        self.0 == other.0
    }
}

pub trait Indexable<NodeData, ConnectionData>: Sized {
    fn get(graph: &Graph<NodeData, ConnectionData>, idx: Index<Self>) -> Option<&Self>;
    fn get_mut(graph: &mut Graph<NodeData, ConnectionData>, idx: Index<Self>) -> Option<&mut Self>;
}

impl<T> Index<T> {
    pub(crate) fn new(idx: usize) -> Self {
        Index(idx, PhantomData)
    }
}

impl<NodeData, ConnectionData> Indexable<NodeData, ConnectionData> for Node<NodeData, ConnectionData> {
    fn get(graph: &Graph<NodeData, ConnectionData>, idx: Index<Self>) -> Option<&Self> {
        graph.nodes.get(idx)
    }

    fn get_mut(graph: &mut Graph<NodeData, ConnectionData>, idx: Index<Self>) -> Option<&mut Self> {
        graph.nodes.get_mut(idx)
    }
}

impl<NodeData, ConnectionData> Indexable<NodeData, ConnectionData> for Connection<NodeData, ConnectionData> {
    fn get(graph: &Graph<NodeData, ConnectionData>, idx: Index<Self>) -> Option<&Self> {
        graph.connections.get(idx)
    }

    fn get_mut(graph: &mut Graph<NodeData, ConnectionData>, idx: Index<Self>) -> Option<&mut Self> {
        graph.connections.get_mut(idx)
    }
}

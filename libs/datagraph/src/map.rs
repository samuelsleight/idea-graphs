use vec_map::VecMap;

use crate::{
    index::Index
};

pub(crate) struct IndexMap<T> {
    map: VecMap<T>,
    idx: usize
}

impl<T> IndexMap<T> {
    pub(crate) fn new() -> Self {
        IndexMap {
            map: VecMap::new(),
            idx: 0
        }
    }

    pub(crate) fn insert(&mut self, data: T) -> Index<T> {
        self.map.insert(self.idx, data);
        self.idx += 1;
        Index::new(self.idx - 1)
    }

    pub(crate) fn get(&self, idx: Index<T>) -> Option<&T> {
        self.map.get(idx.0)
    }

    pub(crate) fn get_mut(&mut self, idx: Index<T>) -> Option<&mut T> {
        self.map.get_mut(idx.0)
    }
}

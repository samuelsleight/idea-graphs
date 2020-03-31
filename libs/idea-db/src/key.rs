use std::marker::PhantomData;

pub struct Key<T> {
    pub(crate) key: i64,

    _phantom: PhantomData<T>
}

impl<T> Key<T> {
    pub(crate) fn new(key: i64) -> Self {
        Self {
            key,
            _phantom: PhantomData
        }
    }
}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            _phantom: self._phantom
        }
    }
}

impl<T> Copy for Key<T> {}

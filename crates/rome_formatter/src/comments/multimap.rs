use std::collections::HashMap;

/// Multimap implementation that uses a shared vector to store the values for each key.
///
/// The map uses a single vector to store the values of all keys together with a map
/// that stores the the value range for each key. The upside of using a single vector for all
/// values is that it avoids allocating a new vector for every element. The downside is that the values
/// for a key must all be appended in order.
#[derive(Clone)]
pub(super) struct AppendOnlyMultiMap<K, V> {
    index: HashMap<K, ValueRange>,
    values: Vec<V>,
}

impl<K: std::hash::Hash + Eq, V> AppendOnlyMultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            values: Vec::new(),
        }
    }

    /// Appends the `value` to the `key`'s values.
    ///
    /// # Panics
    /// If `key` is already present in the map but other keys have been inserted since it was initially inserted.
    pub fn append(&mut self, key: K, value: V) {
        if let Some(range) = self.index.get_mut(&key) {
            assert_eq!(self.values.len(), range.end());

            self.values.push(value);
            range.increment_end();
        } else {
            let range = ValueRange::single(self.values.len());
            self.values.push(value);
            self.index.insert(key, range);
        }
    }

    /// Returns an iterator over all the keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.index.keys()
    }

    /// Returns a slice of the values associated with `key`.
    pub fn get(&self, key: &K) -> &[V] {
        if let Some(range) = self.index.get(key) {
            &self.values[range.start()..range.end()]
        } else {
            &[]
        }
    }
}

impl<K, V> Default for AppendOnlyMultiMap<K, V> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            index: HashMap::new(),
        }
    }
}

impl<K, V> std::fmt::Debug for AppendOnlyMultiMap<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_map();

        for (key, range) in &self.index {
            builder.entry(&key, &&self.values[range.start()..range.end()]);
        }

        builder.finish()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct ValueRange {
    start: u32,
    end: u32,
}

impl ValueRange {
    fn single(position: usize) -> Self {
        Self {
            start: position as u32,
            end: (position + 1) as u32,
        }
    }

    fn start(&self) -> usize {
        self.start as usize
    }

    fn end(&self) -> usize {
        self.end as usize
    }

    fn increment_end(&mut self) {
        self.end += 1;
    }
}

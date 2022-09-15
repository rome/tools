use countme::Count;
use rustc_hash::FxHashMap;
use std::fmt::{Debug, Formatter};
use std::iter::FusedIterator;
use std::num::NonZeroU32;
use std::ops::Range;

/// An optimized multi-map implementation for storing leading, dangling, and trailing parts for a key.
///
/// The implementation allows to store leading, dangling, and trailing parts for every key.
/// A general purpose multimap implementation would allocate a new [Vec] for the leading, dangling, and trailing
/// parts for every key.
///
/// This implementation avoids this based on the assumptions that, for the vast majority of insertions:
/// * The leading, dangling, and trailing parts for a key are inserted in this order.
/// * The parts are inserted per key. For example, the parts for key one are inserted before the parts for key 2, and so on.
///
/// Parts that are inserted in compilation with these assumption are stored in a single [Vec] that is
/// shared between all keys to significantly reduce the number of allocated [Vec]s.
///
/// The implementation does support insertions that don't comply with the above mentioned rules but
/// these come at a performance penalty:
/// * It requires allocating three [Vec], one for the leading, dangling, and trailing parts.
/// * Already inserted parts must be copied over (by cloning) into these newly allocated [Vec]s.
/// * Resolving the slices for every part requires an extra level of indirection.
///
/// ## Panics
/// Panics when storing the `u32::MAX` part.
pub(super) struct CommentsMap<K, V> {
    /// Lookup table to retrieve the entry for a key.
    index: FxHashMap<K, Entry>,

    /// Flat array storing all the values that have been inserted in order.
    parts: Vec<V>,

    /// Vector of vectors. Stores a triple of leading, dangling, and trailing [Vec] for every out of order entry.
    ///
    /// The length of this vec is guaranteed to always be a multiple of 3 where the leading parts are
    /// at offset 0, the dangling at offset 1, and the trailing parts at offset 2.
    out_of_order: Vec<Vec<V>>,
}

impl<K: std::hash::Hash + Eq, V> CommentsMap<K, V> {
    pub fn new() -> Self {
        Self {
            index: FxHashMap::default(),
            parts: Vec::new(),
            out_of_order: Vec::new(),
        }
    }

    /// Pushes a leading part for `key`.
    pub fn push_leading(&mut self, key: K, part: V)
    where
        V: Clone,
    {
        match self.index.get_mut(&key) {
            None => {
                let start = self.parts.len();
                self.parts.push(part);

                self.index.insert(
                    key,
                    Entry::InOrder(InOrderEntry::leading(start..self.parts.len())),
                );
            }

            // Has only leading comments and no elements have been pushed since
            Some(Entry::InOrder(entry))
                if entry.trailing_start.is_none() && self.parts.len() == entry.range().end =>
            {
                self.parts.push(part);
                entry.increment_leading_range();
            }

            Some(Entry::OutOfOrder(entry)) => {
                let leading = &mut self.out_of_order[entry.leading_index()];
                leading.push(part);
            }

            Some(entry) => {
                let out_of_order =
                    Self::entry_to_out_of_order(entry, &self.parts, &mut self.out_of_order);
                self.out_of_order[out_of_order.leading_index()].push(part);
            }
        }
    }

    /// Pushes a dangling part for `key`
    pub fn push_dangling(&mut self, key: K, part: V)
    where
        V: Clone,
    {
        match self.index.get_mut(&key) {
            None => {
                let start = self.parts.len();
                self.parts.push(part);

                self.index.insert(
                    key,
                    Entry::InOrder(InOrderEntry::dangling(start..self.parts.len())),
                );
            }

            // Has leading and dangling comments and its comments are at the end of values
            Some(Entry::InOrder(entry))
                if entry.trailing_end.is_none() && self.parts.len() == entry.range().end =>
            {
                self.parts.push(part);
                entry.increment_dangling_range();
            }

            Some(Entry::OutOfOrder(entry)) => {
                let dangling = &mut self.out_of_order[entry.dangling_index()];
                dangling.push(part);
            }

            Some(entry) => {
                let out_of_order =
                    Self::entry_to_out_of_order(entry, &self.parts, &mut self.out_of_order);
                self.out_of_order[out_of_order.dangling_index()].push(part);
            }
        }
    }

    /// Pushes a trailing part for `key`.
    pub fn push_trailing(&mut self, key: K, part: V)
    where
        V: Clone,
    {
        match self.index.get_mut(&key) {
            None => {
                let start = self.parts.len();
                self.parts.push(part);

                self.index.insert(
                    key,
                    Entry::InOrder(InOrderEntry::trailing(start..self.parts.len())),
                );
            }

            // Its comments are at the end
            Some(Entry::InOrder(entry)) if entry.range().end == self.parts.len() => {
                self.parts.push(part);
                entry.increment_trailing_range();
            }

            Some(Entry::OutOfOrder(entry)) => {
                let trailing = &mut self.out_of_order[entry.trailing_index()];
                trailing.push(part);
            }

            Some(entry) => {
                let out_of_order =
                    Self::entry_to_out_of_order(entry, &self.parts, &mut self.out_of_order);
                self.out_of_order[out_of_order.trailing_index()].push(part);
            }
        }
    }

    #[cold]
    fn entry_to_out_of_order<'a>(
        entry: &'a mut Entry,
        values: &[V],
        out_of_order: &mut Vec<Vec<V>>,
    ) -> &'a mut OutOfOrderEntry
    where
        V: Clone,
    {
        match entry {
            Entry::InOrder(in_order) => {
                let index = out_of_order.len();

                out_of_order.push(values[in_order.leading_range()].to_vec());
                out_of_order.push(values[in_order.dangling_range()].to_vec());
                out_of_order.push(values[in_order.trailing_range()].to_vec());

                *entry = Entry::OutOfOrder(OutOfOrderEntry {
                    leading_index: index,
                    _count: Count::new(),
                });

                match entry {
                    Entry::InOrder(_) => unreachable!(),
                    Entry::OutOfOrder(out_of_order) => out_of_order,
                }
            }
            Entry::OutOfOrder(entry) => entry,
        }
    }

    /// Retrieves all leading parts of `key`
    pub fn leading(&self, key: &K) -> &[V] {
        match self.index.get(key) {
            None => &[],
            Some(Entry::InOrder(in_order)) => &self.parts[in_order.leading_range()],
            Some(Entry::OutOfOrder(entry)) => &self.out_of_order[entry.leading_index()],
        }
    }

    /// Retrieves all dangling parts of `key`.
    pub fn dangling(&self, key: &K) -> &[V] {
        match self.index.get(key) {
            None => &[],
            Some(Entry::InOrder(in_order)) => &self.parts[in_order.dangling_range()],
            Some(Entry::OutOfOrder(entry)) => &self.out_of_order[entry.dangling_index()],
        }
    }

    /// Retrieves all trailing parts of `key`.
    pub fn trailing(&self, key: &K) -> &[V] {
        match self.index.get(key) {
            None => &[],
            Some(Entry::InOrder(in_order)) => &self.parts[in_order.trailing_range()],
            Some(Entry::OutOfOrder(entry)) => &self.out_of_order[entry.trailing_index()],
        }
    }

    /// Returns `true` if `key` has any leading, dangling, or trailing part.
    pub fn has(&self, key: &K) -> bool {
        self.index.get(key).is_some()
    }

    /// Returns an iterator over all leading, dangling, and trailing parts of `key`.
    pub fn parts(&self, key: &K) -> ValuesIterator<V> {
        match self.index.get(key) {
            None => ValuesIterator::Slice([].iter()),
            Some(Entry::OutOfOrder(entry)) => ValuesIterator::Leading {
                leading: self.out_of_order[entry.leading_index()].iter(),
                dangling: &self.out_of_order[entry.dangling_index()],
                trailing: &self.out_of_order[entry.trailing_index()],
            },
            Some(Entry::InOrder(entry)) => ValuesIterator::Slice(self.parts[entry.range()].iter()),
        }
    }
}

impl<K: std::hash::Hash + Eq, V> Default for CommentsMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> std::fmt::Debug for CommentsMap<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_map();

        for (key, entry) in &self.index {
            builder.entry(&key, &DebugEntry { entry, map: self });
        }

        builder.finish()
    }
}

/// Iterator to iterate over all leading, dangling, and trailing parts of a key.
pub(super) enum ValuesIterator<'a, V> {
    /// The slice into the [CommentsMap::parts] [Vec] if this is an in-order entry or the trailing parts
    /// of an out-of-order entry.
    Slice(std::slice::Iter<'a, V>),

    /// Iterator over the leading parts of an out-of-order entry. Returns the dangling parts, and then the
    /// trailing parts once the leading iterator is fully consumed.
    Leading {
        leading: std::slice::Iter<'a, V>,
        dangling: &'a [V],
        trailing: &'a [V],
    },

    /// Iterator over the dangling parts of an out-of-order entry. Returns the trailing parts
    /// once the leading iterator is fully consumed.
    Dangling {
        dangling: std::slice::Iter<'a, V>,
        trailing: &'a [V],
    },
}

impl<'a, V> Iterator for ValuesIterator<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ValuesIterator::Slice(inner) => inner.next(),

            ValuesIterator::Leading {
                leading,
                dangling,
                trailing,
            } => match leading.next() {
                Some(next) => Some(next),
                None if !dangling.is_empty() => {
                    let mut dangling_iterator = dangling.iter();
                    let next = dangling_iterator.next().unwrap();
                    *self = ValuesIterator::Dangling {
                        dangling: dangling_iterator,
                        trailing,
                    };
                    Some(next)
                }
                None => {
                    let mut trailing_iterator = trailing.iter();
                    let next = trailing_iterator.next();
                    *self = ValuesIterator::Slice(trailing_iterator);
                    next
                }
            },

            ValuesIterator::Dangling { dangling, trailing } => match dangling.next() {
                Some(next) => Some(next),
                None => {
                    let mut trailing_iterator = trailing.iter();
                    let next = trailing_iterator.next();
                    *self = ValuesIterator::Slice(trailing_iterator);
                    next
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            ValuesIterator::Slice(slice) => slice.size_hint(),
            ValuesIterator::Leading {
                leading,
                dangling,
                trailing,
            } => {
                let len = leading.len() + dangling.len() + trailing.len();

                (len, Some(len))
            }
            ValuesIterator::Dangling { dangling, trailing } => {
                let len = dangling.len() + trailing.len();
                (len, Some(len))
            }
        }
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        match self {
            ValuesIterator::Slice(slice) => slice.last(),
            ValuesIterator::Leading {
                leading,
                dangling,
                trailing,
            } => trailing
                .last()
                .or_else(|| dangling.last())
                .or_else(|| leading.last()),
            ValuesIterator::Dangling { dangling, trailing } => {
                trailing.last().or_else(|| dangling.last())
            }
        }
    }
}

impl<V> ExactSizeIterator for ValuesIterator<'_, V> {}

impl<V> FusedIterator for ValuesIterator<'_, V> {}

#[derive(Debug)]
enum Entry {
    InOrder(InOrderEntry),
    OutOfOrder(OutOfOrderEntry),
}

struct DebugEntry<'a, K, V> {
    entry: &'a Entry,
    map: &'a CommentsMap<K, V>,
}

impl<K, V> Debug for DebugEntry<'_, K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let leading = match self.entry {
            Entry::OutOfOrder(entry) => self.map.out_of_order[entry.leading_index()].as_slice(),
            Entry::InOrder(entry) => &self.map.parts[entry.leading_range()],
        };

        let dangling = match self.entry {
            Entry::OutOfOrder(entry) => self.map.out_of_order[entry.dangling_index()].as_slice(),
            Entry::InOrder(entry) => &self.map.parts[entry.dangling_range()],
        };

        let trailing = match self.entry {
            Entry::OutOfOrder(entry) => self.map.out_of_order[entry.trailing_index()].as_slice(),
            Entry::InOrder(entry) => &self.map.parts[entry.trailing_range()],
        };

        let mut list = f.debug_list();

        list.entries(leading.iter().map(DebugValue::Leading));
        list.entries(dangling.iter().map(DebugValue::Dangling));
        list.entries(trailing.iter().map(DebugValue::Trailing));

        list.finish()
    }
}

enum DebugValue<'a, V> {
    Leading(&'a V),
    Dangling(&'a V),
    Trailing(&'a V),
}

impl<V> Debug for DebugValue<'_, V>
where
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugValue::Leading(leading) => f.debug_tuple("Leading").field(leading).finish(),
            DebugValue::Dangling(dangling) => f.debug_tuple("Dangling").field(dangling).finish(),
            DebugValue::Trailing(trailing) => f.debug_tuple("Trailing").field(trailing).finish(),
        }
    }
}

#[derive(Debug)]
struct InOrderEntry {
    /// Index into the [CommentsMap::parts] vector where the leading parts of this entry start
    leading_start: PartIndex,

    /// Index into the [CommentsMap::parts] vector where the dangling parts (and, thus, the leading parts end) start.
    dangling_start: PartIndex,

    /// Index into the [CommentsMap::parts] vector where the trailing parts (and, thus, the dangling parts end) of this entry start
    trailing_start: Option<PartIndex>,

    /// Index into the [CommentsMap::parts] vector where the trailing parts of this entry end
    trailing_end: Option<PartIndex>,

    _count: Count<InOrderEntry>,
}

impl InOrderEntry {
    fn leading(range: Range<usize>) -> Self {
        InOrderEntry {
            leading_start: PartIndex::from_len(range.start),
            dangling_start: PartIndex::from_len(range.end),
            trailing_start: None,
            trailing_end: None,
            _count: Count::new(),
        }
    }

    fn dangling(range: Range<usize>) -> Self {
        let start = PartIndex::from_len(range.start);
        InOrderEntry {
            leading_start: start,
            dangling_start: start,
            trailing_start: Some(PartIndex::from_len(range.end)),
            trailing_end: None,
            _count: Count::new(),
        }
    }

    fn trailing(range: Range<usize>) -> Self {
        let start = PartIndex::from_len(range.start);
        InOrderEntry {
            leading_start: start,
            dangling_start: start,
            trailing_start: Some(start),
            trailing_end: Some(PartIndex::from_len(range.end)),
            _count: Count::new(),
        }
    }

    fn increment_leading_range(&mut self) {
        assert!(
            self.trailing_start.is_none(),
            "Can't extend the leading range for an in order entry with dangling comments."
        );

        self.dangling_start.increment();
    }

    fn increment_dangling_range(&mut self) {
        assert!(
            self.trailing_end.is_none(),
            "Can't extend the dangling range for an in order entry with trailing comments."
        );

        match &mut self.trailing_start {
            Some(start) => start.increment(),
            None => self.trailing_start = Some(self.dangling_start.incremented()),
        }
    }

    fn increment_trailing_range(&mut self) {
        match (self.trailing_start, &mut self.trailing_end) {
            // Already has some trailing comments
            (Some(_), Some(end)) => end.increment(),
            // Has dangling comments only
            (Some(start), None) => self.trailing_end = Some(start.incremented()),
            // Has leading comments only
            (None, None) => {
                self.trailing_start = Some(self.dangling_start);
                self.trailing_end = Some(self.dangling_start.incremented())
            }
            (None, Some(_)) => {
                unreachable!()
            }
        }
    }

    fn leading_range(&self) -> Range<usize> {
        self.leading_start.value()..self.dangling_start.value()
    }

    fn dangling_range(&self) -> Range<usize> {
        match self.trailing_start {
            None => self.dangling_start.value()..self.dangling_start.value(),
            Some(trailing_start) => self.dangling_start.value()..trailing_start.value(),
        }
    }

    fn trailing_range(&self) -> Range<usize> {
        match (self.trailing_start, self.trailing_end) {
            (Some(trailing_start), Some(trailing_end)) => {
                trailing_start.value()..trailing_end.value()
            }
            // Only dangling comments
            (Some(trailing_start), None) => trailing_start.value()..trailing_start.value(),
            (None, Some(_)) => {
                panic!("Trailing end shouldn't be set if trailing start is none");
            }
            (None, None) => self.dangling_start.value()..self.dangling_start.value(),
        }
    }

    fn range(&self) -> Range<usize> {
        self.leading_start.value()
            ..self
                .trailing_end
                .or(self.trailing_start)
                .unwrap_or(self.dangling_start)
                .value()
    }
}

#[derive(Debug)]
struct OutOfOrderEntry {
    /// Index into the [CommentsMap::out_of_order] vector at which offset the leaading vec is stored.
    leading_index: usize,
    _count: Count<OutOfOrderEntry>,
}

impl OutOfOrderEntry {
    const fn leading_index(&self) -> usize {
        self.leading_index
    }

    const fn dangling_index(&self) -> usize {
        self.leading_index + 1
    }

    const fn trailing_index(&self) -> usize {
        self.leading_index + 2
    }
}

/// Index into the [CommentsMap::parts] vector.
///
/// Stores the index as a [NonZeroU32], starting at 1 instead of 0 so that
/// `size_of::<PartIndex>() == size_of::<Option<PartIndex>>()`.
///
/// This means, that only `u32 - 1` parts can be stored. This should be sufficient for storing comments
/// because: Comments have length of two or more bytes because they consist of a start and end character sequence (`#` + new line, `/*` and `*/`).
/// Thus, a document with length `u32` can have at most `u32::MAX / 2` comment-parts.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct PartIndex(NonZeroU32);

impl PartIndex {
    fn from_len(value: usize) -> Self {
        Self(NonZeroU32::try_from(value as u32 + 1).unwrap())
    }

    fn value(&self) -> usize {
        (u32::from(self.0) - 1) as usize
    }

    fn increment(&mut self) {
        *self = self.incremented();
    }

    fn incremented(&self) -> PartIndex {
        PartIndex(NonZeroU32::new(self.0.get() + 1).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::comments::map::CommentsMap;

    static EMPTY: [i32; 0] = [];

    #[test]
    fn leading_dangling_trailing() {
        let mut map = CommentsMap::new();

        map.push_leading("a", 1);
        map.push_dangling("a", 2);
        map.push_dangling("a", 3);
        map.push_trailing("a", 4);

        assert_eq!(map.parts, vec![1, 2, 3, 4]);

        assert_eq!(map.leading(&"a"), &[1]);
        assert_eq!(map.dangling(&"a"), &[2, 3]);
        assert_eq!(map.trailing(&"a"), &[4]);

        assert!(map.has(&"a"));

        assert_eq!(
            map.parts(&"a").copied().collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn dangling_trailing() {
        let mut map = CommentsMap::new();

        map.push_dangling("a", 1);
        map.push_dangling("a", 2);
        map.push_trailing("a", 3);

        assert_eq!(map.parts, vec![1, 2, 3]);

        assert_eq!(map.leading(&"a"), &EMPTY);
        assert_eq!(map.dangling(&"a"), &[1, 2]);
        assert_eq!(map.trailing(&"a"), &[3]);

        assert!(map.has(&"a"));

        assert_eq!(map.parts(&"a").copied().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn trailing() {
        let mut map = CommentsMap::new();

        map.push_trailing("a", 1);
        map.push_trailing("a", 2);

        assert_eq!(map.parts, vec![1, 2]);

        assert_eq!(map.leading(&"a"), &EMPTY);
        assert_eq!(map.dangling(&"a"), &EMPTY);
        assert_eq!(map.trailing(&"a"), &[1, 2]);

        assert!(map.has(&"a"));

        assert_eq!(map.parts(&"a").copied().collect::<Vec<_>>(), vec![1, 2]);
    }

    #[test]
    fn empty() {
        let map = CommentsMap::<&str, i32>::default();

        assert_eq!(map.parts, Vec::<i32>::new());

        assert_eq!(map.leading(&"a"), &EMPTY);
        assert_eq!(map.dangling(&"a"), &EMPTY);
        assert_eq!(map.trailing(&"a"), &EMPTY);

        assert!(!map.has(&"a"));

        assert_eq!(
            map.parts(&"a").copied().collect::<Vec<_>>(),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn multiple_keys() {
        let mut map = CommentsMap::new();

        map.push_leading("a", 1);
        map.push_dangling("b", 2);
        map.push_trailing("c", 3);
        map.push_leading("d", 4);
        map.push_dangling("d", 5);
        map.push_trailing("d", 6);

        assert_eq!(map.parts, &[1, 2, 3, 4, 5, 6]);

        assert_eq!(map.leading(&"a"), &[1]);
        assert_eq!(map.dangling(&"a"), &EMPTY);
        assert_eq!(map.trailing(&"a"), &EMPTY);
        assert_eq!(map.parts(&"a").copied().collect::<Vec<_>>(), vec![1]);

        assert_eq!(map.leading(&"b"), &EMPTY);
        assert_eq!(map.dangling(&"b"), &[2]);
        assert_eq!(map.trailing(&"b"), &EMPTY);
        assert_eq!(map.parts(&"b").copied().collect::<Vec<_>>(), vec![2]);

        assert_eq!(map.leading(&"c"), &EMPTY);
        assert_eq!(map.dangling(&"c"), &EMPTY);
        assert_eq!(map.trailing(&"c"), &[3]);
        assert_eq!(map.parts(&"c").copied().collect::<Vec<_>>(), vec![3]);

        assert_eq!(map.leading(&"d"), &[4]);
        assert_eq!(map.dangling(&"d"), &[5]);
        assert_eq!(map.trailing(&"d"), &[6]);
        assert_eq!(map.parts(&"d").copied().collect::<Vec<_>>(), vec![4, 5, 6]);
    }

    #[test]
    fn dangling_leading() {
        let mut map = CommentsMap::new();

        map.push_dangling("a", 1);
        map.push_leading("a", 2);
        map.push_dangling("a", 3);
        map.push_trailing("a", 4);

        assert_eq!(map.leading(&"a"), [2]);
        assert_eq!(map.dangling(&"a"), [1, 3]);
        assert_eq!(map.trailing(&"a"), [4]);

        assert_eq!(
            map.parts(&"a").copied().collect::<Vec<_>>(),
            vec![2, 1, 3, 4]
        );

        assert!(map.has(&"a"));
    }

    #[test]
    fn trailing_leading() {
        let mut map = CommentsMap::new();

        map.push_trailing("a", 1);
        map.push_leading("a", 2);
        map.push_dangling("a", 3);
        map.push_trailing("a", 4);

        assert_eq!(map.leading(&"a"), [2]);
        assert_eq!(map.dangling(&"a"), [3]);
        assert_eq!(map.trailing(&"a"), [1, 4]);

        assert_eq!(
            map.parts(&"a").copied().collect::<Vec<_>>(),
            vec![2, 3, 1, 4]
        );

        assert!(map.has(&"a"));
    }

    #[test]
    fn trailing_dangling() {
        let mut map = CommentsMap::new();

        map.push_trailing("a", 1);
        map.push_dangling("a", 2);
        map.push_trailing("a", 3);

        assert_eq!(map.leading(&"a"), &EMPTY);
        assert_eq!(map.dangling(&"a"), &[2]);
        assert_eq!(map.trailing(&"a"), &[1, 3]);

        assert_eq!(map.parts(&"a").copied().collect::<Vec<_>>(), vec![2, 1, 3]);

        assert!(map.has(&"a"));
    }

    #[test]
    fn keys_out_of_order() {
        let mut map = CommentsMap::new();

        map.push_leading("a", 1);
        map.push_dangling("b", 2);
        map.push_leading("a", 3);

        map.push_trailing("c", 4);
        map.push_dangling("b", 5);

        map.push_leading("d", 6);
        map.push_trailing("c", 7);

        assert_eq!(map.leading(&"a"), &[1, 3]);
        assert_eq!(map.dangling(&"b"), &[2, 5]);
        assert_eq!(map.trailing(&"c"), &[4, 7]);

        assert!(map.has(&"a"));
        assert!(map.has(&"b"));
        assert!(map.has(&"c"));
    }
}

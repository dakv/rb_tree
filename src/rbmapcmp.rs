use crate::{RBMapWithCmp, Comparator, ComparatorWrapper, RBTreeWithCmp};
use crate::mapper::SimpleMapper;
use std::fmt;
use std::iter::FusedIterator;

impl<K, V, F: Comparator<K>> RBMapWithCmp<K, V, F> {
    /// Creates and returns a new, empty RBMapWithCmp
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// map.insert("Hello", "World");
    /// assert_eq!(map.remove("Hello").unwrap(), "World");
    /// ```
    pub fn new(cmp: F) -> RBMapWithCmp<K, V, F> {
        RBMapWithCmp { map: RBTreeWithCmp::new(ComparatorWrapper::new(cmp)) }
    }

    /// Returns true if the map contains an entry
    /// for key, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// assert!(!map.contains_key("Hello"));
    /// map.insert("Hello", "world");
    /// assert!(map.contains_key(&"Hello"));
    /// ```
    pub fn contains_key(&self, key: K) -> bool {
        match self.map.get(&SimpleMapper::new(key, None)) {
            None => false,
            Some(v) => v.is_some(),
        }
    }

    /// Returns an option containing a reference
    /// to the value associated with this key,
    /// or none if this key does not have an associated
    /// value.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// assert!(map.get("Hello").is_none());
    /// map.insert("Hello", "world");
    /// assert_eq!(map.get(&"Hello").unwrap(), &"world");
    /// ```
    pub fn get(&self, key: K) -> Option<&V> {
        self.map.get(&SimpleMapper::new(key, None)).map(|v| v.as_ref())
    }

    /// Returns an option containing a reference
    /// to the key-value pair associated with this
    /// key, or none if this key does not have an
    /// associated value.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// assert!(map.get("Hello").is_none());
    /// map.insert("Hello", "world");
    /// assert_eq!(map.get_pair(&"Hello").unwrap(), (&"Hello", &"world"));
    /// ```
    pub fn get_pair(&self, key: K) -> Option<(&K, &V)> {
        self.map
            .get(&SimpleMapper::new(key, None))
            .map(|v| (v.key(), v.as_ref()))
    }

    /// Returns an option containing a reference
    /// to the key-value pair associated with this
    /// key of which the value is mutable.
    /// Returns none if this key does not have an
    /// associated value.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// assert!(map.get("Hello").is_none());
    /// map.insert("Hello", "world");
    /// assert_eq!(map.get_pair(&"Hello").unwrap(), (&"Hello", &"world"));
    /// ```
    pub fn get_pair_mut(&mut self, key: K) -> Option<(&K, &mut V)> {
        self.map
            .get_mut(&SimpleMapper::new(key, None))
            .map(|v| v.mut_pair())
    }

    /// Returns an option containing a mutable
    /// reference to the value associated with this
    /// key, or none if this key does not have an associated
    /// value.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// assert!(map.get("Hello").is_none());
    /// map.insert("Hello", "world");
    /// *map.get_mut(&"Hello").unwrap() = "world!";
    /// assert_eq!(map.get(&"Hello").unwrap(), &"world!");
    /// ```
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.map
            .get_mut(&SimpleMapper::new(key, None))
            .map(|v| v.as_mut())
    }

    /// Inserts a value to associate with the given key
    /// into the map, returning the previously-stored key-value
    /// pair if one existed, None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// map.insert("Hello", "world");
    /// map.insert("Foo", "bar");
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn insert(&mut self, key: K, val: V) -> Option<(K, V)> {
        self.map
            .replace(SimpleMapper::new(key, Some(val)))
            .map(|v| v.consume())
    }

    /// Returns true if there are no key-value pairs
    /// stored in this RBMapWithCmp, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// assert!(map.is_empty());
    /// map.insert(1, 2);
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.len() == 0
    }

    /// Returns the number of key-value pairs stored
    /// in this RBMapWithCmp.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// assert_eq!(map.len(), 0);
    /// map.insert(1, 1);
    /// assert_eq!(map.len(), 1);
    /// map.insert(2, 4);
    /// assert_eq!(map.len(), 2);
    /// map.remove(2);
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Removes the key-value pair associated with key,
    /// if one exists, and returns the associated value,
    /// or None if the pair did not exist.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// assert!(map.remove(2).is_none());
    /// map.insert(2, 4);
    /// assert_eq!(map.remove(2).unwrap(), 4);
    /// ```
    pub fn remove(&mut self, key: K) -> Option<V> {
        self.map
            .take(&SimpleMapper::new(key, None))
            .map(|v| v.consume().1)
    }

    /// An iterator that visits all key-value
    /// pairs in their key's partialord order.
    /// # Example:
    /// ```
    /// use rb_tree::{RBMapWithCmp, TestComparator};
    ///
    /// let mut map = RBMapWithCmp::new(TestComparator);
    /// map.insert(1, 1);
    /// map.insert(2, 4);
    /// map.insert(3, 9);
    ///
    /// let mut pairs = map.iter();
    /// assert_eq!(pairs.next().unwrap(), (&1, &1));
    /// assert_eq!(pairs.next().unwrap(), (&2, &4));
    /// assert_eq!(pairs.next().unwrap(), (&3, &9));
    /// assert_eq!(pairs.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            pos: 0,
            ordered: self.ordered(),
        }
    }

    fn ordered(&self) -> Vec<(&K, &V)> {
        self.map.iter().map(|m| (m.key(), m.as_ref())).collect()
    }
}

pub struct IntoIter<K, V, F: 'static + Comparator<K>> {
    tree: RBTreeWithCmp<SimpleMapper<K, V>, ComparatorWrapper<K, F>>,
}

impl<K, V, F: Comparator<K>> Iterator for IntoIter<K, V, F> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        self.tree.pop().map(|v| v.consume())
    }
}

impl<K, V, F: Comparator<K>> ExactSizeIterator for IntoIter<K, V, F> {
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl<K, V, F: Comparator<K>> FusedIterator for IntoIter<K, V, F> {}

impl<K, V, F: Comparator<K>> IntoIterator for RBMapWithCmp<K, V, F> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, F>;

    fn into_iter(self) -> IntoIter<K, V, F> {
        IntoIter { tree: self.map }
    }
}


pub struct Iter<'a, K, V> {
    pos: usize,
    ordered: Vec<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        match self.ordered.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Some(*v)
            }
            None => None,
        }
    }
}

impl<K: fmt::Debug, V: fmt::Debug, F: Comparator<K>> fmt::Debug for RBMapWithCmp<K, V, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.ordered())
    }
}
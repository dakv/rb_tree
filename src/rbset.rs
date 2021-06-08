use crate::helpers::write_to_level;
use crate::{RBSet, RBTreeWithCmp};
use std::fmt::{Debug, Display, Formatter, Result};

impl<K, F: Fn(&K, &K) -> std::cmp::Ordering> RBSet<K, F> {
    /// Creates and returns a new, empty RBSet
    /// # Example:
    /// ```
    /// use rb_tree::RBSet;
    ///
    /// let mut set = RBSet::new(|a: &String, b: &String| { a.cmp(b) });
    /// set.insert("Hello".to_string());
    /// assert!(set.remove(&"Hello".to_string()).is_some());
    /// ```
    pub fn new(f: F) -> RBSet<K, F> {
        RBSet {
            map: RBTreeWithCmp::new(f),
        }
    }

    /// # Example:
    /// ```
    /// use rb_tree::RBSet;
    ///
    /// let mut set = RBSet::new(|a: &String, b: &String| { a.cmp(b) });
    /// set.insert("Hello".to_string());
    /// set.insert("Foo".to_string());
    /// assert_eq!(set.len(), 2);
    /// ```
    pub fn insert(&mut self, key: K) -> Option<K> {
        self.map.replace(key)
    }

    /// # Example:
    /// ```
    /// use rb_tree::RBSet;
    ///
    /// let mut set = RBSet::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// assert!(set.remove(&2).is_none());
    /// set.insert(2);
    /// assert!(set.remove(&2).is_some());
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<K> {
        self.map.take(key)
    }

    /// Returns the number of entries stored in this RBSet.
    /// # Example:
    /// ```
    /// use rb_tree::RBSet;
    ///
    /// let mut set = RBSet::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// assert_eq!(set.len(), 0);
    /// set.insert(1);
    /// assert_eq!(set.len(), 1);
    /// set.insert(2);
    /// assert_eq!(set.len(), 2);
    /// set.remove(&2);
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn ordered(&self) -> Vec<&K> {
        self.map.iter().collect()
    }

    /// # Example:
    /// ```
    /// use rb_tree::RBSet;
    ///
    /// let mut t = RBSet::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// t.insert(2);
    /// t.insert(1);
    /// t.insert(3);
    /// assert_eq!(t.pop().unwrap(), 1);
    /// ```
    pub fn pop(&mut self) -> Option<K> {
        self.map.pop()
    }

    /// # Example:
    /// ```
    /// use rb_tree::RBSet;
    ///
    /// let mut set = RBSet::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// set.insert(1);
    /// set.insert(2);
    /// set.insert(3);
    ///
    /// let mut pairs = set.iter();
    /// assert_eq!(pairs.next().unwrap(), &1);
    /// assert_eq!(pairs.next().unwrap(), &2);
    /// assert_eq!(pairs.next().unwrap(), &3);
    /// assert_eq!(pairs.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<K> {
        Iter {
            pos: 0,
            ordered: self.ordered(),
        }
    }
}

pub struct Iter<'a, K> {
    pos: usize,
    ordered: Vec<&'a K>,
}

impl<'a, K> Iterator for Iter<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<&'a K> {
        match self.ordered.get(self.pos) {
            Some(v) => {
                self.pos += 1;
                Some(*v)
            }
            None => None,
        }
    }
}

impl<K: Debug, F: Fn(&K, &K) -> std::cmp::Ordering> Debug for RBSet<K, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut levels = Vec::new();
        write_to_level(&self.map.root, "".to_string(), 0, &mut levels);
        let mut f_string = "".to_string();
        for i in 0..levels.len() {
            f_string += &levels[i];
            if i != levels.len() - 1 {
                f_string += "\n";
            }
        }
        write!(f, "{}", f_string)
    }
}

impl<K: Debug, F: Fn(&K, &K) -> std::cmp::Ordering> Display for RBSet<K, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.ordered())
    }
}

pub struct IntoIter<K, F: Fn(&K, &K) -> std::cmp::Ordering> {
    tree: RBSet<K, F>,
}

impl<K, F: Fn(&K, &K) -> std::cmp::Ordering> Iterator for IntoIter<K, F> {
    type Item = K;

    fn next(&mut self) -> Option<K> {
        self.tree.pop()
    }
}

impl<K, F: Fn(&K, &K) -> std::cmp::Ordering> IntoIterator for RBSet<K, F> {
    type Item = K;
    type IntoIter = IntoIter<K, F>;

    fn into_iter(self) -> IntoIter<K, F> {
        IntoIter { tree: self }
    }
}

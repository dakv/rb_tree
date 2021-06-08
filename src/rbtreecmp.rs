use crate::helpers::{insert_left_down, ordered_insertion, write_to_level};
use crate::node::Colour::Black;
use crate::node::Node;
use crate::node::Node::Leaf;
use crate::RBTreeWithCmp;
use std::fmt::{Debug, Display, Formatter, Result};

impl<T, F: Fn(&T, &T) -> std::cmp::Ordering> RBTreeWithCmp<T, F> {
    /// Creates and returns a new RBTreeWithCmp.
    /// # Example:
    /// ```
    /// use rb_tree::RBTreeWithCmp;
    ///
    /// let mut t = RBTreeWithCmp::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// t.insert(3);
    /// t.insert(2);
    /// assert_eq!(t.take(&2).unwrap(), 2);
    /// ```
    pub fn new(f: F) -> RBTreeWithCmp<T, F> {
        RBTreeWithCmp {
            root: Leaf(Black),
            contained: 0,
            cmp: f,
        }
    }

    /// Inserts a new element into the RBTreeWithCmp.
    /// Returns true if this item was not already
    /// in the tree, and false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTreeWithCmp;
    ///
    /// let mut t = RBTreeWithCmp::new(|a: &String, b: &String| { a.cmp(b) });
    /// assert_eq!(t.insert("Hello".to_string()), true);
    /// assert_eq!(t.insert("Hello".to_string()), false);
    /// ```
    pub fn insert(&mut self, val: T) -> bool {
        match self.root.insert(val, &self.cmp) {
            Some(_) => false,
            None => {
                self.contained += 1;
                true
            }
        }
    }

    /// Returns a vector presenting the contained
    /// elements of the RBTreeWithCmp in the order by which
    /// they are prioritised (that is, in the in-order
    /// tree traversal order).
    /// # Example:
    /// ```
    /// use rb_tree::RBTreeWithCmp;
    ///
    /// let mut t = RBTreeWithCmp::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// t.insert(3);
    /// t.insert(1);
    /// t.insert(2);
    /// let order = t.ordered();
    /// assert_eq!(*order[1], 2);
    /// ```
    pub fn ordered(&self) -> Vec<&T> {
        let mut order = Vec::new();
        ordered_insertion(&self.root, &mut order);
        order
    }

    /// Returns the number of elements contained in the tree.
    /// # Example:
    /// ```
    /// use rb_tree::RBTreeWithCmp;
    ///
    /// let mut t = RBTreeWithCmp::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// t.insert(3);
    /// t.insert(1);
    /// t.insert(2);
    /// assert_eq!(t.len(), 3);
    /// t.remove(&2);
    /// assert_eq!(t.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.contained
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Removes an item the tree. Returns the matching item
    /// if it was contained in the tree, None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTreeWithCmp;
    ///
    /// let mut t = RBTreeWithCmp::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// t.insert(4);
    /// t.insert(2);
    /// assert_eq!(t.take(&2).unwrap(), 2);
    /// assert_eq!(t.len(), 1);
    /// assert_eq!(t.take(&2), None);
    /// ```
    pub fn take(&mut self, val: &T) -> Option<T> {
        match self.root.remove(val, &self.cmp) {
            Some(v) => {
                self.contained -= 1;
                Some(v)
            }
            None => None,
        }
    }

    /// Removes an item the tree. Returns true
    /// if it was contained in the tree, false otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTreeWithCmp;
    ///
    /// let mut t = RBTreeWithCmp::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// t.insert(4);
    /// t.insert(2);
    /// assert_eq!(t.remove(&2), true);
    /// assert_eq!(t.len(), 1);
    /// assert_eq!(t.remove(&2), false);
    /// ```
    pub fn remove(&mut self, val: &T) -> bool {
        match self.root.remove(val, &self.cmp) {
            Some(_) => {
                self.contained -= 1;
                true
            }
            None => false,
        }
    }

    /// Removes the item at the front of the priority
    /// queue that the RBTree represents if any elements
    /// are present, or None otherwise.
    /// # Example:
    /// ```
    /// use rb_tree::RBTreeWithCmp;
    ///
    /// let mut t = RBTreeWithCmp::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// t.insert(2);
    /// t.insert(1);
    /// t.insert(3);
    /// assert_eq!(t.pop().unwrap(), 1);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        match self.root.pop(false) {
            Some(v) => {
                self.contained -= 1;
                Some(v)
            }
            None => None,
        }
    }

    /// Returns an iterator over the elements
    /// contained in this RBTreeWithCmp.
    /// # Example:
    /// ```
    /// use rb_tree::RBTreeWithCmp;
    ///
    /// let mut t = RBTreeWithCmp::new(|a: &u64, b: &u64| { a.cmp(b) });
    /// t.insert(2);
    /// t.insert(3);
    /// t.insert(1);
    /// t.insert(4);
    /// let mut count = 0;
    /// for i in t.iter() {
    ///     count += 1;
    ///     assert_eq!(count, *i);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<T> {
        let mut ordered = Vec::new();
        insert_left_down(&self.root, &mut ordered);
        Iter {
            remaining: self.len(),
            ordered,
        }
    }

    /// # Example:
    /// ```
    /// use rb_tree::RBTreeWithCmp;
    ///
    /// let mut t = RBTreeWithCmp::new(|a: &String, b: &String| { a.cmp(b) });
    /// assert_eq!(t.replace("Hello".to_string()), None);
    /// assert_eq!(t.replace("Hello".to_string()), Some("Hello".to_string()));
    /// ```
    pub fn replace(&mut self, val: T) -> Option<T> {
        match self.root.insert(val, &self.cmp) {
            Some(v) => Some(v),
            None => {
                self.contained += 1;
                None
            }
        }
    }
}

pub struct Iter<'a, T> {
    remaining: usize,
    ordered: Vec<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let next = match self.ordered.pop() {
            Some(n) => n,
            None => return None,
        };
        self.remaining -= 1;
        insert_left_down(next.get_right(), &mut self.ordered);
        Some(next.value().unwrap())
    }
}

impl<T: Debug, F: Fn(&T, &T) -> std::cmp::Ordering> Debug for RBTreeWithCmp<T, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut levels = Vec::new();
        write_to_level(&self.root, "".to_string(), 0, &mut levels);
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

impl<T: Debug, F: Fn(&T, &T) -> std::cmp::Ordering> Display for RBTreeWithCmp<T, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.ordered())
    }
}

pub struct IntoIter<T, F: Fn(&T, &T) -> std::cmp::Ordering> {
    tree: RBTreeWithCmp<T, F>,
}

impl<T, F: Fn(&T, &T) -> std::cmp::Ordering> Iterator for IntoIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.tree.pop()
    }
}

impl<T, F: Fn(&T, &T) -> std::cmp::Ordering> IntoIterator for RBTreeWithCmp<T, F> {
    type Item = T;
    type IntoIter = IntoIter<T, F>;

    fn into_iter(self) -> IntoIter<T, F> {
        IntoIter { tree: self }
    }
}

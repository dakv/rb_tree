mod node;
#[cfg(feature = "map")]
pub mod rbmap;
#[cfg(feature = "set")]
pub mod rbtree;
#[macro_use]
#[cfg(feature = "queue")]
pub mod rbqueue;
mod helpers;
#[cfg(feature = "map")]
mod mapper;
#[cfg(test)]
mod rbtree_tests;
#[cfg(test)]
mod stress_test;

mod rbtreecmp;
#[cfg(test)]
mod rbtreecmp_tests;

mod rbmapcmp;
#[cfg(test)]
mod rbmapcmp_tests;
mod rbset;
#[cfg(test)]
mod rbset_test;

use crate::mapper::SimpleMapper;
#[cfg(feature = "map")]
use mapper::Mapper;
use node::Node;
use std::marker::PhantomData;
use std::rc::Rc;

/// A map implemented using a red black tree to
/// store key-value pairs.
#[cfg(feature = "map")]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
pub struct RBMap<K: PartialOrd, V> {
    map: RBTree<Mapper<K, V>>,
}

#[derive(Clone)]
struct ComparatorWrapper<K, F: Comparator<K>> {
    cmp: Rc<F>,
    _k: PhantomData<K>,
}

impl<K, F: Comparator<K>> ComparatorWrapper<K, F> {
    pub fn new(cmp: F) -> Self {
        Self {
            cmp: Rc::new(cmp),
            _k: PhantomData,
        }
    }
}

impl<K, V, F: 'static + Comparator<K>> Comparator<SimpleMapper<K, V>> for ComparatorWrapper<K, F> {
    fn cmp(&self) -> Box<dyn Fn(&SimpleMapper<K, V>, &SimpleMapper<K, V>) -> std::cmp::Ordering> {
        let f = self.cmp.clone();
        Box::new(move |a: &SimpleMapper<K, V>, b: &SimpleMapper<K, V>| f.cmp()(a.key(), b.key()))
    }
}

#[derive(Clone)]
pub struct RBMapWithCmp<K, V, F: 'static + Comparator<K>> {
    map: RBTreeWithCmp<SimpleMapper<K, V>, ComparatorWrapper<K, F>>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
pub struct RBSet<K, F: Comparator<K>> {
    map: RBTreeWithCmp<K, F>,
}

/// A red black tree that can be used to store
/// elements sorted by their PartialOrd provided
/// ordering.
#[cfg(feature = "set")]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
pub struct RBTree<T: PartialOrd> {
    root: Node<T>,
    contained: usize,
}

pub trait Comparator<T> {
    fn cmp(&self) -> Box<dyn Fn(&T, &T) -> std::cmp::Ordering>;
}

pub struct TestComparator;

impl<T> Comparator<T> for TestComparator
where
    T: Ord,
{
    fn cmp(&self) -> Box<dyn Fn(&T, &T) -> std::cmp::Ordering> {
        Box::new(|a: &T, b: &T| a.cmp(b))
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
pub struct RBTreeWithCmp<T, F: Comparator<T>> {
    root: Node<T>,
    cmp: F,
    contained: usize,
}

/// A priority queue implemented using a red black
/// tree. The ordering supplied must satisfy the assymetry
/// and transitivity rules as outlined by  the dorumentation
/// of std::cmp::PartialOrd.
#[cfg(feature = "queue")]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
pub struct RBQueue<T, P>
where
    P: Fn(&T, &T) -> std::cmp::Ordering,
{
    root: Node<T>,
    contained: usize,
    cmp: P,
}

/// Returns an RBTree containing the items
/// given separated by commas.
/// # Example:
/// ```
/// use rb_tree::{RBTree, new_set};
///
/// let t1 = new_set!('b', 'a', 'd', 'c');
/// let t2 = new_set!('d', 'f', 'e', 'c');
///
/// let mut in_both = t1.intersection(&t2);
/// assert_eq!(in_both.next().unwrap(), &'c');
/// assert_eq!(in_both.next().unwrap(), &'d');
/// assert_eq!(in_both.next(), None);
/// ```
#[cfg(feature = "set")]
#[macro_export]
macro_rules! new_set {
    ( $($v:expr),* ) => {{
        let mut t = RBTree::new();
        $(
            t.insert($v);
        )*
        t
    }};
}

/// Returns an RBQueue that prioritises on given
/// closure and contains the comma-separated
/// elements following it.
/// # Example:
/// use rb_tree::{RBQueue, new_queue};
///
/// let mut q = new_queue!(|l, r| {
/// match l - r {
///     i32::MIN..=-1_i32 => Greater,
///     0 => Equal,
///     1_i32..=i32::MAX => Less
/// }
/// }; 1, 2, 3, 4);
/// assert_eq!(q.pop().unwrap(), 4);
/// assert_eq!(q.pop().unwrap(), 3);
/// assert_eq!(q.pop().unwrap(), 2);
/// assert_eq!(q.pop().unwrap(), 1);
/// assert_eq!(q.pop(), None);
/// ```
#[cfg(feature = "queue")]
#[macro_export]
macro_rules! new_queue {
    ($comp:expr; $($v:expr),*) => {{
        let mut q = RBQueue::new($comp);
        $(q.insert($v);)*
        q
    }};
}

/// Allows the creation of a queue using C-like
/// comparison values. That is to say, `cmp`
/// should return a value less than, equal to,
/// or greater than 0 when `l` should be placed
/// before, is equal to, or be placed after `r`
/// respectively.
///
/// `cmp` should be a function that takes two values
/// from the queue and returns an integer (i8)
/// providing the information as above.
///
/// # Example:
/// ```
/// # #[macro_use(new_c_queue)]
/// # extern crate rb_tree;
/// # use rb_tree::RBQueue;
/// # fn main() {
/// let mut q = new_c_queue!(|l: &i64, r| (r - l));
/// q.insert(1);
/// q.insert(2);
/// q.insert(3);
/// assert_eq!(q.ordered(), [&3, &2, &1]);
/// # }
/// ```
///
/// # Example:
/// ```
/// # #[macro_use(new_c_queue)]
/// # extern crate rb_tree;
/// # use rb_tree::RBQueue;
/// # fn main() {
/// let q = new_c_queue!(|l: &i64, r| (r - l); 1, 2, 3);
/// assert_eq!(q.ordered(), [&3, &2, &1]);
/// # }
/// ```
#[cfg(feature = "queue")]
#[macro_export]
macro_rules! new_c_queue {
    ($cmp:expr) => {
        RBQueue::new(move |l, r| {
            let comp = Box::new($cmp);
            match comp(l, r) as i8 {
                -128i8 ..= -1 => std::cmp::Ordering::Less,
                0 => std::cmp::Ordering::Equal,
                1 ..= 127i8 => std::cmp::Ordering::Greater
            }
        })
    };

    ($cmp:expr; $($v:expr),*) => {{
        let mut q = RBQueue::new(move |l, r| {
            let comp = Box::new($cmp);
            match comp(l, r) as i8 {
                -128i8 ..= -1 => std::cmp::Ordering::Less,
                0 => std::cmp::Ordering::Equal,
                1 ..= 127i8 => std::cmp::Ordering::Greater
            }
        });
        $(
            q.insert($v);
        )*
        q
    }};
}

/// Returns an RBMap containing the (key, value)
/// pairs separated by commas.
/// # Example:
/// ```
/// use rb_tree::{RBMap, new_map};
///
/// let m = new_map!((1, 'a'), (2, 'b'), (3, 'c'));
/// assert_eq!(m.get(&1).unwrap(), &'a');
/// assert_eq!(m.get(&2).unwrap(), &'b');
/// assert_eq!(m.get(&3).unwrap(), &'c');
/// ```
#[cfg(feature = "map")]
#[macro_export]
macro_rules! new_map {
    ( $(($k:expr, $v:expr)),* ) => {{
        let mut m = RBMap::new();
        $(
            m.insert($k, $v);
        )*
        m
    }};
}

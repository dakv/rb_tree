use crate::{RBMapWithCmp, TestComparator};

#[test]
fn test_print() {
    let mut t = RBMapWithCmp::new(TestComparator);
    t.insert(2, 2);
    t.insert(3, 3);
    t.insert(1, 1);
    t.insert(4, 4);
    assert_eq!(format!("{:?}", t), "[(1, 1), (2, 2), (3, 3), (4, 4)]");
    assert_eq!(t.len(), 4);
}

#[test]
fn test_iter() {
    let mut t = RBMapWithCmp::new(TestComparator);
    t.insert(2, 2);
    t.insert(3, 3);
    t.insert(1, 1);
    t.insert(4, 4);
    let mut count = 0;
    for (k, _v) in t.iter() {
        count += 1;
        assert_eq!(count, *k);
    }
    let mut count = 0;
    for (k, _v) in t {
        count += 1;
        assert_eq!(count, k);
    }
}

#[test]
fn test_rev() {
    let mut t = RBMapWithCmp::new(TestComparator);
    t.insert(2, 2);
    t.insert(3, 3);
    t.insert(1, 1);
    t.insert(4, 4);
    let mut count = 0;
    for (k, _v) in t.iter() {
        count += 1;
        assert_eq!(count, *k);
    }

    for (&k, _v) in t.ordered().iter().rev() {
        assert_eq!(count, k);
        count -= 1;
    }
}

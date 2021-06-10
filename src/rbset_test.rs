use crate::{RBSet, TestComparator};

#[test]
fn test_print() {
    let mut t = RBSet::new(TestComparator {});
    t.insert(2);
    t.insert(3);
    t.insert(1);
    t.insert(4);
    assert_eq!(format!("{}", t), "[1, 2, 3, 4]");
    assert_eq!(t.len(), 4);
    assert_eq!(
        format!("{:?}", t),
        "B:2\n2->B:1 2->B:3\n1->___ 1->___ 3->___ 3->R:4\n4->___ 4->___"
    );
}

#[test]
fn test_iter() {
    let mut t = RBSet::new(TestComparator {});
    t.insert(2);
    t.insert(3);
    t.insert(1);
    t.insert(4);
    let mut count = 0;
    for i in t.iter() {
        count += 1;
        assert_eq!(count, *i);
    }
}

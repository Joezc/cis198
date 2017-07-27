#[cfg(test)]

use first::BTree;

#[test]
fn test_insert() {
    let mut bst = BTree::new();

    for i in 0 .. 1000 {
        bst.insert(i);
    }

    for i in 0 .. 1000 {
        assert!(bst.search(i));
    }
}

#[test]
fn test_search() {
    let mut bst = BTree::new();
    bst.insert(1);
    bst.insert(10);
    bst.insert(4);
    bst.insert(7);
    bst.insert(2);

    assert!(bst.search(1));
    assert!(!bst.search(11));
    assert!(bst.search(7));
    assert!(bst.search(10));
    assert!(!bst.search(13));
    assert!(bst.search(4));
    assert!(bst.search(2));
    assert!(bst.search(10));
}

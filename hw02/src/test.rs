#[cfg(test)]

use first::BST;

#[test]
fn test_insert() {
    let mut bst = BST::new();

    for i in 0 .. 10000 {
        assert!(bst.insert(i));
    }

    for i in 0 .. 10000 {
        assert!(bst.search(i));
    }
}

#[test]
fn test_search() {
    let mut bst = BST::new();
    assert!(bst.insert(1));
    assert!(bst.insert(10));
    assert!(bst.insert(4));
    assert!(bst.insert(7));
    assert!(bst.insert(2));

    assert!(bst.search(1));
    assert!(!bst.search(11));
    assert!(bst.search(7));
    assert!(bst.search(10));
    assert!(!bst.search(13));
    assert!(bst.search(4));
    assert!(bst.search(2));
    assert!(bst.search(10));
}

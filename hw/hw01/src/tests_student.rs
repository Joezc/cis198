#![cfg(test)]

use problem1::{sum};

// problem1

#[test]
fn test_sum_large() {
    let array = [100; 100];
    assert_eq!(sum(&array), 10000);
}

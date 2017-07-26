/// Compute the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut ans: i32 = 0;
    for x in slice {
        ans += *x;
    }

    ans
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    for e in vs {
        if !ans.contains(e) {
            ans.push(*e);
        }
    }

    ans
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut ans = Vec::new();
    for e in vs {
        if pred(*e) {
            ans.push(*e);
        }
    }

    ans
}

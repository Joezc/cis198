/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    // TODO
    let mut ans = Vec::new();
    let mut flag;
    for i in 2..n+1 {
        flag = 0;
        for j in 2..i {
            if i%j == 0 {
                flag = 1;
            }
        }
        if flag == 0 {
            ans.push(i);
        }
    }

    ans
}

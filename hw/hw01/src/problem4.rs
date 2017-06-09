/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut ans = Vec::new();
    if num_discs == 1 {
        ans.push((src, dst));
    } else {
        ans.append(&mut hanoi(num_discs-1, src, dst, aux));
        ans.push((src,dst));
        ans.append(&mut hanoi(num_discs-1, aux, src, dst));
    }
    ans
}

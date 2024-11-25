#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); m],
    }

    let cd = cd
        .into_iter()
        .flat_map(|(c, d)| vec![(c, d), (d, c)])
        .collect::<HashSet<_>>();

    let mut ans = false;

    for p in (0..n).permutations(n) {
        ans |= ab.iter().all(|&(a, b)| cd.contains(&(p[a], p[b])));
    }

    println!("{}", if ans { "Yes" } else { "No" });
}

#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let comp = a
        .iter()
        .sorted()
        .dedup()
        .copied()
        .enumerate()
        .map(|(i, ai)| (ai, i))
        .collect::<HashMap<_, _>>();

    use ac_library::ModInt998244353 as Mint;

    let mint2 = Mint::new(2);
    let mut bit = ac_library::FenwickTree::new(comp.len(), Mint::new(0));
    let mut ans = Mint::new(0);

    for i in 0..n {
        ans += bit.sum(..=comp[&a[i]]) * mint2.pow(i as u64);
        bit.add(comp[&a[i]], mint2.pow((i + 1) as u64).inv());
    }

    println!("{}", ans);
}

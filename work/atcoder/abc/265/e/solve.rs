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
        n: usize, m: usize,
        a: isize, b: isize, c: isize, d: isize, e: isize, f: isize,
        xy: [(isize, isize); m],
    }

    let moves = [(a, b), (c, d), (e, f)];
    let xy = xy.into_iter().collect::<HashSet<_>>();

    use ac_library::ModInt998244353 as Mint;

    let mut dp = hashmap! {};
    dp.insert((0, 0), Mint::new(1));

    for _ in 0..n {
        let mut next_dp = hashmap! {};

        for ((x, y), z) in dp {
            for (dx, dy) in moves {
                let (nx, ny) = (x + dx, y + dy);
                if !xy.contains(&(nx, ny)) {
                    let value = next_dp.entry((nx, ny)).or_insert(Mint::new(0));
                    *value += z;
                }
            }
        }

        dp = next_dp;
    }

    println!("{}", dp.values().sum::<Mint>());
}

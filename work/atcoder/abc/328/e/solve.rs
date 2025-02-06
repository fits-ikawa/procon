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
        n: usize, m: usize, k: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let mut ans = usize::MAX;

    for edges in uvw.iter().combinations(n - 1) {
        let mut uf = ac_library::Dsu::new(n);
        let mut cost = 0;

        for &(u, v, w) in edges {
            uf.merge(u, v);
            cost += w;
        }

        if uf.groups().len() == 1 {
            ans = ans.min(cost % k);
        }
    }

    println!("{}", ans);
}

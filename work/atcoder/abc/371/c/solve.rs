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
        n: usize,
        mg: usize,
        uv: [(Usize1, Usize1); mg],
        mh: usize,
        ab: [(Usize1, Usize1); mh],
    }

    let edges_g = uv
        .into_iter()
        .flat_map(|(u, v)| [(u, v), (v, u)])
        .collect::<HashSet<_>>();

    let edges_h = ab
        .into_iter()
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .collect::<HashSet<_>>();

    let mut a = vec![vec![0; n]; n];

    for i in 0..n - 1 {
        for j in i + 1..n {
            input! {
                aij: usize,
            }
            a[i][j] = aij;
            a[j][i] = aij;
        }
    }

    let mut ans = usize::MAX;

    for p in (0..n).permutations(n) {
        let mut cost = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                if edges_g.contains(&(i, j)) != edges_h.contains(&(p[i], p[j])) {
                    cost += a[p[i]][p[j]];
                }
            }
        }
        ans = ans.min(cost);
    }

    println!("{}", ans);
}

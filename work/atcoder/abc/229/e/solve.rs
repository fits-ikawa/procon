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
        ab: [(Usize1, Usize1); m],
    }

    let mut edges = vec![vec![]; n];

    for (a, b) in ab {
        edges[a.min(b)].push((a, b));
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut cnt = 0;
    let mut ans = vec![];

    for i in (0..n).rev() {
        ans.push(cnt);

        cnt += 1;

        for &(a, b) in &edges[i] {
            if !uf.same(a, b) {
                uf.merge(a, b);
                cnt -= 1;
            }
        }
    }

    println!("{}", ans.iter().rev().join("\n"));
}

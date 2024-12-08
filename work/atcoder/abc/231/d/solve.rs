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
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut count = vec![0; n];

    for (a, b) in ab {
        count[a] += 1;
        count[b] += 1;

        if count[a] >= 3 || count[b] >= 3 || uf.same(a, b) {
            println!("No");
            return;
        }

        uf.merge(a, b);
    }

    println!("Yes");
}

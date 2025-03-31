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
        d: usize,
        n: usize,
        lr: [(Usize1, Usize1); n],
    }

    let mut imos = vec![0; d + 1];

    for (l, r) in lr {
        imos[l] += 1;
        imos[r + 1] -= 1;
    }

    let ans = imos.iter().cumsum::<isize>().collect_vec();

    println!("{}", ans[..d].iter().join("\n"));
}

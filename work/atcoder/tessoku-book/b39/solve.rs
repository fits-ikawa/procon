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
        n: usize, d: usize,
        mut xy: [(usize, usize); n],
    }

    xy.sort_by_key(|&(x, _)| x);

    let mut heap = BinaryHeap::new();
    let mut j = 0;
    let mut ans = 0;

    for i in 1..=d {
        while j < n && xy[j].0 <= i {
            heap.push(xy[j].1);
            j += 1;
        }
        ans += heap.pop().unwrap_or(0);
    }

    println!("{}", ans);
}

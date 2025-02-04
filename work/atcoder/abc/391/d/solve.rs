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
        n: usize, w: usize,
        xy: [(Usize1, Usize1); n],
        q: usize,
        ta: [(usize, Usize1); q],
    }

    let mut col = vec![btreeset! {}; w];

    for (i, (x, y)) in xy.into_iter().enumerate() {
        col[x].insert((y, i));
    }

    let mut vanish = vec![usize::MAX; n];

    'outer: loop {
        let mut collect = vec![];
        let mut max_t = 0;

        for i in 0..w {
            if col[i].is_empty() {
                break 'outer;
            }
            let (t, i) = col[i].pop_first().unwrap();
            collect.push(i);
            max_t = max_t.max(t);
        }

        for i in collect {
            vanish[i] = max_t + 1;
        }
    }

    for (t, a) in ta {
        println!("{}", if vanish[a] > t { "Yes" } else { "No" });
    }
}

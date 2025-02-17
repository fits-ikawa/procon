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
        h: usize, w: usize, m: usize,
        tax: [(usize, Usize1, usize); m],
    }

    let mut row = (0..h).collect::<HashSet<_>>();
    let mut col = (0..w).collect::<HashSet<_>>();
    let mut map = btreemap! {};

    for &(t, a, x) in tax.iter().rev() {
        if t == 1 {
            if row.contains(&a) {
                let value = map.entry(x).or_insert(0);
                *value += col.len();
                row.remove(&a);
            }
        } else if col.contains(&a) {
            let value = map.entry(x).or_insert(0);
            *value += row.len();
            col.remove(&a);
        }
    }

    let mut ans = VecDeque::new();
    let mut not_zero = 0;

    for (k, v) in map {
        if k > 0 && v > 0 {
            ans.push_back((k, v));
            not_zero += v;
        }
    }

    let zeros = h * w - not_zero;

    if zeros > 0 {
        ans.push_front((0, zeros));
    }

    println!("{}", ans.len());

    for (k, v) in ans {
        println!("{} {}", k, v);
    }
}

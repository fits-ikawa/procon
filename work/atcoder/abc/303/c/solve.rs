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
        _n: usize, m: usize, mut h:isize, k:isize,
        s: Chars,
        xy: [(isize, isize); m],
    }

    let mut xy = xy.into_iter().collect::<HashSet<_>>();

    let mut cur = (0, 0);

    for si in s {
        let (dx, dy) = match si {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };

        h -= 1;

        if h < 0 {
            println!("No");
            return;
        }

        cur = (cur.0 + dx, cur.1 + dy);

        if xy.contains(&cur) && h < k {
            h = k;
            xy.remove(&cur);
        }
    }

    println!("Yes");
}

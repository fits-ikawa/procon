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
        n: usize, k: usize,
        a: [Usize1; k],
        xy: [(isize, isize); n],
    }

    let a = a.into_iter().collect::<HashSet<_>>();

    let (light, nolight): (Vec<_>, Vec<_>) = xy.iter().enumerate().partition_map(|(i, &(x, y))| {
        if a.contains(&i) {
            Either::Left((x, y))
        } else {
            Either::Right((x, y))
        }
    });

    let mut r = 0;

    for &(x1, y1) in &nolight {
        let mut s = isize::MAX;
        for &(x2, y2) in &light {
            s = s.min((x2 - x1).pow(2) + (y2 - y1).pow(2));
        }
        r = r.max(s);
    }

    println!("{}", (r as f64).sqrt());
}

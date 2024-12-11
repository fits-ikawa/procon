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
        a: [usize; n],
    }

    let (c2, c3) = a.iter().fold((usize::MAX, usize::MAX), |acc, &ai| {
        let (c2, c3, _) = div(ai);
        (acc.0.min(c2), acc.1.min(c3))
    });

    let mut residual = vec![];
    let mut ans = 0;

    for ai in a {
        let (d2, d3, y) = div(ai);
        if d2 < c2 || d3 < c3 {
            println!("-1");
            return;
        }

        ans += d2 - c2;
        ans += d3 - c3;
        residual.push(y);
    }

    if residual.iter().all_equal() {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn div(x: usize) -> (usize, usize, usize) {
    let mut x = x;
    let mut count2 = 0;
    let mut count3 = 0;

    while x % 2 == 0 {
        x /= 2;
        count2 += 1;
    }

    while x % 3 == 0 {
        x /= 3;
        count3 += 1;
    }

    (count2, count3, x)
}

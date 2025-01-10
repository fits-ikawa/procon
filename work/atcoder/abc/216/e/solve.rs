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
    // 二分探索（解説 AC）
    input! {
        n: usize, mut k: usize,
        a: [usize; n],
    }

    let mut left = 0;
    let mut right = 3 * 10_usize.pow(9);

    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut l = 0;

        for &ai in &a {
            l += ai.saturating_sub(mid - 1);
        }

        if l <= k {
            right = mid;
        } else {
            left = mid;
        }
    }

    let mut l = 0;
    let mut ans = 0;

    for &ai in &a {
        let diff = ai.saturating_sub(right - 1);
        l += diff;
        ans += (ai + right) * diff / 2;
    }

    ans += (right - 1) * (k - l);

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, mut k: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.reverse();
    a.push(0);

    let mut ans = 0;

    for i in 0..n {
        let diff = a[i] - a[i + 1];
        if diff == 0 {
            continue;
        }

        if diff * (i + 1) <= k {
            ans += (((a[i + 1] + 1 + a[i]) * diff) / 2) * (i + 1);
            k -= diff * (i + 1);
        } else {
            let l = k / (i + 1);
            ans += (((a[i] + a[i] + 1 - l) * l) / 2) * (i + 1);
            ans += (a[i] - l) * (k % (i + 1));
            break;
        }
    }

    println!("{}", ans);
}

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
        n: usize,
        a: [usize; n],
    }

    let mut p = vec![0; n];
    let mut border = 2;

    for i in 1..n {
        if a[i] >= border {
            p[i] = p[i - 1] + 1;
            border += 1;
        } else {
            p[i] = a[i] - 1;
            border = a[i] + 1;
        }
    }

    let mut q = vec![0; n];
    let mut border = 2;

    for i in (0..n - 1).rev() {
        if a[i] >= border {
            q[i] = q[i + 1] + 1;
            border += 1;
        } else {
            q[i] = a[i] - 1;
            border = a[i] + 1;
        }
    }

    let mut ans = 0;

    for i in 0..n {
        ans = ans.max(p[i].min(q[i]) + 1);
    }

    println!("{}", ans);
}

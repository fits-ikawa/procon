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
        mut lr: [(usize, usize); n],
    }

    lr.sort();

    let mut r_max = 0;
    let mut ans = vec![0];

    for &(l, r) in &lr {
        if r_max < l {
            ans.push(r_max);
            ans.push(l);
        }
        r_max = r_max.max(r);
    }
    ans.push(r_max);

    let ans = ans[2..].chunks(2).collect_vec();

    println!("{}", ans.iter().map(|x| x.iter().join(" ")).join("\n"));
}

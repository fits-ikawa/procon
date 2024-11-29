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
        mut a: [isize; n],
    }

    a.sort();
    a.reverse();

    let mut even = vec![];
    let mut odd = vec![];

    for ai in a {
        if ai % 2 == 0 {
            even.push(ai);
        } else {
            odd.push(ai);
        }
    }

    let mut ans = -1;

    if even.len() >= 2 {
        ans = ans.max(even[0] + even[1]);
    }
    if odd.len() >= 2 {
        ans = ans.max(odd[0] + odd[1]);
    }

    println!("{}", ans);
}

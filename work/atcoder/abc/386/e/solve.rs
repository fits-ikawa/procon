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
        n: usize, k: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    if k <= n / 2 {
        for c in a.into_iter().combinations(k) {
            ans = ans.max(c.into_iter().fold(0, |acc, ci| acc ^ ci));
        }
    } else {
        let xorsum = a.iter().fold(0, |acc, &ai| acc ^ ai);

        for c in a.into_iter().combinations(n - k) {
            ans = ans.max(xorsum ^ c.into_iter().fold(0, |acc, ci| acc ^ ci));
        }
    }

    println!("{}", ans);
}

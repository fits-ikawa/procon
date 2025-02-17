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
        s: Chars,
    }

    let mut ans = 0;

    for comb in (0..s.len()).combinations(3) {
        let (i, j, k) = (comb[0], comb[1], comb[2]);
        if s[i] == 'A' && s[j] == 'B' && s[k] == 'C' && k - j == j - i {
            ans += 1;
        }
    }

    println!("{}", ans);
}

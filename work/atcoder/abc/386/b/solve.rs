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
        mut s: Chars,
    }

    let mut ans = 0;

    while !s.is_empty() {
        let n = s.len();
        if n >= 2 && s[n - 1] == '0' && s[n - 2] == '0' {
            s.pop();
            s.pop();
        } else {
            s.pop();
        }
        ans += 1;
    }

    println!("{}", ans);
}

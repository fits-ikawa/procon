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
        n: isize, a: isize, b: isize,
        s: Chars,
    }

    let mut ans = isize::MAX;

    for rotate in 0..n {
        let mut change = 0;
        for i in 0..n / 2 {
            let l = ((rotate + i) % n) as usize;
            let r = ((rotate + (n - 1 - i)) % n) as usize;
            if s[l] != s[r] {
                change += 1;
            }
        }
        ans = ans.min(rotate * a + change * b);
    }

    println!("{}", ans);
}

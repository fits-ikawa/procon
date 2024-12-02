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
        n: usize, m: usize,
    }

    let mut s = vec![];

    for _ in 0..m {
        input! {
            c: usize,
            a: [usize; c],
        }

        let a = a.into_iter().collect::<HashSet<_>>();
        s.push(a);
    }

    let mut ans = 0;

    for k in 1..=m {
        for c in s.iter().combinations(k) {
            let mut valid = true;
            for x in 1..=n {
                valid &= c.iter().any(|ci| ci.contains(&x));
            }
            if valid {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

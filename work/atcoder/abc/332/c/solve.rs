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
        n: isize, m: isize,
        s: Chars,
    }

    let mut left = -1;
    let mut right = n;

    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut plain = m;
        let mut logo = mid;

        let mut f = || {
            for &si in &s {
                match si {
                    '0' => {
                        plain = m;
                        logo = mid;
                    }
                    '1' => {
                        if plain > 0 {
                            plain -= 1;
                        } else if logo > 0 {
                            logo -= 1;
                        } else {
                            return false;
                        }
                    }
                    '2' => {
                        if logo > 0 {
                            logo -= 1;
                        } else {
                            return false;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            true
        };

        if f() {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}

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
        n: usize, tp: Chars,
        s: [Chars; n],
    }

    let mut ans = vec![];

    for (i, si) in s.into_iter().enumerate() {
        if si.len().abs_diff(tp.len()) > 1 {
            continue;
        }

        let valid = if si.len() == tp.len() {
            (0..si.len()).filter(|&j| si[j] != tp[j]).count() <= 1
        } else {
            let (a, b) = if si.len() < tp.len() {
                (&si, &tp)
            } else {
                (&tp, &si)
            };

            let mut valid = true;
            let mut missed = 0;

            for i in 0..a.len() {
                if a[i] != b[i + missed] {
                    missed += 1;
                    if missed >= 2 || a[i] != b[i + missed] {
                        valid = false;
                        break;
                    }
                }
            }

            valid
        };

        if valid {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.len());
    if !ans.is_empty() {
        println!("{}", ans.iter().join(" "));
    }
}

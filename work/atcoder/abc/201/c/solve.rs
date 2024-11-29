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
        s: Chars,
    }

    let mut included = hashset![];
    let mut nums = vec![];

    for (i, &si) in s.iter().enumerate() {
        match si {
            'o' => {
                included.insert(i);
                nums.push(i);
            }
            '?' => {
                nums.push(i);
            }
            _ => (),
        }
    }

    if included.len() > 4 || nums.is_empty() {
        println!("0");
        return;
    }

    let ans = iproduct!(nums.iter(), nums.iter(), nums.iter(), nums.iter())
        .filter(|(&p0, &p1, &p2, &p3)| {
            let used = hashset! {p0, p1, p2, p3};
            included.difference(&used).count() == 0
        })
        .count();

    println!("{}", ans);
}

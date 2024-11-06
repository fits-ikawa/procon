#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        s: String,
    }

    let chars: HashSet<_> = s.replace("ch", "#").chars().collect();

    println!(
        "{}",
        if chars.is_subset(&hashset! {'#', 'o', 'k', 'u'}) {
            "YES"
        } else {
            "NO"
        }
    );
}

#[allow(dead_code)]
fn solve() {
    input! {
        s: String,
    }

    let is_choku = s
        .replace("ch", "#")
        .replace(['o', 'k', 'u'], "#")
        .chars()
        .unique()
        .all_equal_value()
        .map_or(false, |x| x == '#');

    println!("{}", if is_choku { "YES" } else { "NO" });
}

#![allow(unused_imports)]
use itertools::*;
use maplit::*;
use proconio::{marker::*, *};
use std::{cmp::*, collections::*};

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let atcoder: HashSet<_> = "atcoder".chars().collect();

    for pair in s.into_iter().zip(t) {
        match pair {
            (a, b) if a == b => {}
            ('@', b) if atcoder.contains(&b) => {}
            (a, '@') if atcoder.contains(&a) => {}
            _ => {
                println!("You will lose");
                return;
            }
        }
    }

    println!("You can win");
}

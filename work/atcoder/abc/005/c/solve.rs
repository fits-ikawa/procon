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
        t: i32,
        n: usize,
        a: [i32; n],
        m: usize,
        b: [i32; m],
    }

    let mut ai = 0;
    let mut bi = 0;

    'outer: while ai < a.len() && bi < b.len() {
        while b[bi] < a[ai] || a[ai] + t < b[bi] {
            ai += 1;
            if ai >= a.len() {
                break 'outer;
            }
        }
        ai += 1;
        bi += 1;
    }

    println!("{}", if bi == b.len() { "yes" } else { "no" });
}

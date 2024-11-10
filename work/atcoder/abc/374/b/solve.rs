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
        s: String,
        t: String,
    }

    if s == t {
        println!("0");
        return;
    }

    for (i, (si, ti)) in izip!(s.chars(), t.chars()).enumerate() {
        if si != ti {
            println!("{}", i + 1);
            return;
        }
    }

    println!("{}", s.len().min(t.len()) + 1);
}

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
        a: (isize, isize),
        b: (isize, isize),
        c: (isize, isize),
        d: (isize, isize),
    }

    let ab = vec_sub(b, a);
    let bc = vec_sub(c, b);
    let cd = vec_sub(d, c);
    let da = vec_sub(a, d);

    let ans = [ab, bc, cd, da, ab]
        .windows(2)
        .map(|v| cross(v[0], v[1]).signum())
        .all_equal();

    println!("{}", if ans { "Yes" } else { "No" });
}

fn vec_sub(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 - b.0, a.1 - b.1)
}

fn cross(a: (isize, isize), b: (isize, isize)) -> isize {
    a.0 * b.1 - b.0 * a.1
}

#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, p: usize,
        a: [usize; n],
        mut b: [usize; m],
    }

    b.sort();

    let acc = std::iter::once(0)
        .chain(b.iter().copied())
        .cumsum::<usize>()
        .collect_vec();

    let mut ans = 0;

    for &ai in &a {
        if ai >= p {
            ans += p * m;
        } else {
            let pos = b.upper_bound(&(p - ai));
            ans += ai * pos;
            ans += acc[pos];
            ans += p * (m - pos);
        }
    }

    println!("{}", ans);
}

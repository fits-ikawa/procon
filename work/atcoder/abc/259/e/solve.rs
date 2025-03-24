#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        n: usize,
    }

    let mut m = vec![];
    let mut pe = vec![];

    let mut lcm = btreemap! {};

    for _ in 0..n {
        input! {
            mi: usize,
            pei: [(usize, usize); mi],
        }

        for &(p, e) in &pei {
            let value = lcm.entry(p).or_insert(0);
            *value = (*value).max(e);
        }

        m.push(mi);
        pe.push(pei);
    }

    let mut cnt = btreemap! {};

    for i in 0..n {
        for j in 0..m[i] {
            let (p, e) = pe[i][j];
            let value = cnt.entry(p).or_insert(0);

            if e == lcm[&p] {
                *value += 1;
            }
        }
    }

    let mut ans = 0;

    for i in 0..n {
        if (0..m[i]).any(|j| {
            let (p, e) = pe[i][j];
            e == lcm[&p] && cnt[&p] == 1
        }) {
            ans += 1;
        }
    }

    println!("{}", (ans + 1).min(n));
}

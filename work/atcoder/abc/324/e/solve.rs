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
        n: usize, t: Chars,
        s: [Chars; n],
    }

    let t_rev = t.iter().rev().copied().collect_vec();

    let mut fcnt = vec![];
    let mut bcnt = vec![];

    for si in &s {
        let mut i = 0;
        let mut j = 0;
        while i < si.len() && j < t.len() {
            if si[i] == t[j] {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }
        fcnt.push(j);

        let si_rev = si.iter().rev().copied().collect_vec();
        let mut i = 0;
        let mut j = 0;
        while i < si_rev.len() && j < t_rev.len() {
            if si_rev[i] == t_rev[j] {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }
        bcnt.push(j);
    }

    bcnt.sort();

    let mut ans = 0;

    for c in fcnt {
        ans += n - bcnt.lower_bound(&(t.len() - c));
    }

    println!("{}", ans);
}

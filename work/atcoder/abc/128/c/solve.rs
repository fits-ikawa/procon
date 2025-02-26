#![allow(clippy::comparison_chain)]
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
        n: usize, m: usize,
    }

    let mut k = vec![];
    let mut s = vec![];

    for _ in 0..m {
        input! {
            ki: usize, si: [Usize1; ki],
        }

        k.push(ki);
        s.push(si.into_iter().collect::<HashSet<_>>());
    }

    input! {
        p: [usize; m],
    }

    let mut ans = 0;

    for set in (0..n).powerset() {
        let set = set.into_iter().collect::<HashSet<_>>();
        let mut all_on = true;

        for i in 0..m {
            all_on &= set.intersection(&s[i]).count() % 2 == p[i];
        }

        if all_on {
            ans += 1;
        }
    }

    println!("{}", ans);
}

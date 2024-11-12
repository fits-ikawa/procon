#![allow(unused_imports)]
use ac_library::{Additive, Max, Segtree};
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
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize, k:usize,
            a: [usize; n],
            b: [usize; n],
        }

        let ab = izip!(a, b).sorted().collect_vec();
        let mut bsub = BinaryHeap::new();
        let mut bsum = 0;
        let mut ans = usize::MAX;

        for (ai, bi) in ab {
            if bsub.len() == k - 1 {
                ans = ans.min(ai * (bsum + bi));
            }
            bsub.push(bi);
            bsum += bi;
            if bsub.len() > k - 1 {
                bsum -= bsub.pop().unwrap();
            }
        }

        println!("{}", ans);
    }
}

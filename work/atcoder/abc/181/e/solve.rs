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
        n: usize, m: usize,
        mut h: [isize; n],
        w: [isize; m],
    }

    h.sort();

    let acc1 = h
        .iter()
        .enumerate()
        .map(|(i, &hi)| if i % 2 == 0 { hi } else { -hi })
        .cumsum::<isize>()
        .collect_vec();

    let acc2 = h
        .iter()
        .enumerate()
        .map(|(i, &hi)| if i % 2 == 0 { -hi } else { hi })
        .cumsum::<isize>()
        .collect_vec();

    let mut ans = isize::MAX;

    for wi in w {
        let t_pos = h.lower_bound(&wi);
        let diff = if t_pos == 0 {
            // 先生が先頭
            -wi + acc1[n - 1]
        } else if t_pos == n {
            // 先生が末尾
            acc2[n - 1] + wi
        } else if t_pos % 2 == 0 {
            // 先生が中間の偶数番目
            acc2[t_pos - 1] - wi + acc1[n - 1] - acc1[t_pos - 1]
        } else {
            // 先生が中間の奇数番目
            acc2[t_pos - 1] + wi + acc1[n - 1] - acc1[t_pos - 1]
        };
        ans = ans.min(diff);
    }

    println!("{}", ans);
}

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
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let acc = std::iter::once(0)
        .chain(a[1..].chunks(2).map(|c| c[1] - c[0]))
        .cumsum::<usize>()
        .collect_vec();

    for (l, r) in lr {
        let lpos = a.upper_bound(&l);
        let rpos = a.upper_bound(&r);

        if lpos == rpos && lpos % 2 == 0 {
            // ひとつの睡眠区間内での範囲
            println!("{}", r - l);
        } else {
            let mut ans = 0;

            // 左の端数
            if lpos % 2 == 0 {
                ans += a[lpos] - l;
            }

            ans += acc[(rpos - 1) / 2] - acc[lpos / 2];

            // 右の端数
            if rpos % 2 == 0 {
                ans += r - a[rpos - 1];
            }

            println!("{}", ans);
        }
    }
}

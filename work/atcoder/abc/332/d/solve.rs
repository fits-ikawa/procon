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
        h: usize, w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }

    let mut ans = usize::MAX;

    for p in (0..h).permutations(h) {
        for q in (0..w).permutations(w) {
            let mut eq = true;

            for i in 0..h {
                for j in 0..w {
                    eq &= a[p[i]][q[j]] == b[i][j];
                }
            }

            if eq {
                let mut pinv = 0;
                let mut qinv = 0;

                for i in 0..h - 1 {
                    for j in i + 1..h {
                        if p[i] > p[j] {
                            pinv += 1;
                        }
                    }
                }
                for i in 0..w - 1 {
                    for j in i + 1..w {
                        if q[i] > q[j] {
                            qinv += 1;
                        }
                    }
                }

                ans = ans.min(pinv + qinv);
            }
        }
    }

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

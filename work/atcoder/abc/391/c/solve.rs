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
        n: usize, q: usize,
    }

    let mut p2h = (0..n).collect_vec();
    let mut cnt = vec![1; n];
    let mut ans = 0;

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    p: Usize1, h: Usize1,
                }

                let nest = p2h[p];

                if cnt[nest] == 2 {
                    ans -= 1;
                }

                if cnt[h] == 1 {
                    ans += 1;
                }

                p2h[p] = h;
                cnt[nest] -= 1;
                cnt[h] += 1;
            }
            2 => {
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}

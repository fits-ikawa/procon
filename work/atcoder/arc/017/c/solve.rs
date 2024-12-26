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
        n: usize, x: usize,
        w: [usize; n],
    }

    let w1 = calc(&w[..n / 2]);
    let w2 = calc(&w[n / 2..]);

    let mut ans = 0;

    for wi in w1 {
        if wi <= x {
            let cnt = w2.upper_bound(&(x - wi)) - w2.lower_bound(&(x - wi));
            ans += cnt;
        }
    }

    println!("{}", ans);
}

fn calc(w: &[usize]) -> Vec<usize> {
    let n = w.len();
    let mut ret = vec![vec![]; n + 1];

    for i in 0_usize..1 << n {
        let mut sum = 0;
        for j in 0..n {
            if i & 1 << j > 0 {
                sum += w[j];
            }
        }
        ret[i.count_ones() as usize].push(sum);
    }

    ret.into_iter().flatten().sorted().collect()
}

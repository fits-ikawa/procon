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
        n: usize, k: usize, p: usize,
        a: [usize; n],
    }

    let a1 = calc(&a[..n / 2]);
    let a2 = calc(&a[n / 2..]);

    let mut ans = 0;

    for i in 0..=(n / 2).min(k) {
        for &ai in &a1[i] {
            if k - i < a2.len() && ai <= p {
                let pos = a2[k - i].upper_bound(&(p - ai));
                ans += pos;
            }
        }
    }

    println!("{}", ans);
}

fn calc(a: &[usize]) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut ret = vec![vec![]; n + 1];

    for i in 0_usize..1 << n {
        let mut sum = 0;
        for j in 0..n {
            if i & 1 << j > 0 {
                sum += a[j];
            }
        }
        ret[i.count_ones() as usize].push(sum);
    }

    for i in 0..=n {
        ret[i].sort();
    }

    ret
}

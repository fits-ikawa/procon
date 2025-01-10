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
        a: [usize; n],
    }

    let a2 = a.iter().chain(a.iter().take(n)).copied().collect_vec();

    let acc = std::iter::once(0)
        .chain(a2.iter().copied())
        .cumsum::<usize>()
        .map(|x| x % m)
        .collect_vec();

    let mut indices = vec![vec![]; m];

    for (i, &acci) in acc.iter().enumerate() {
        indices[acci].push(i);
    }

    let mut ans = 0;

    for idx in indices {
        if idx.len() <= 1 {
            continue;
        }
        for i in 0.. {
            if i >= idx.len() || idx[i] >= n {
                break;
            }

            ans += idx.lower_bound(&(idx[i] + n)) - idx.upper_bound(&idx[i]);
        }
    }

    println!("{}", ans);
}

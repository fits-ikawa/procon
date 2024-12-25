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

    let asum = a.iter().sum::<usize>();
    let cnt = a.iter().copied().counts();

    let mut keys = cnt.keys().sorted().copied().collect_vec();
    keys.extend(keys.clone());

    let mut table = vec![];
    let mut ans = usize::MAX;

    for i in 0..keys.len() - 1 {
        table.push(keys[i]);
        if table.len() == m || (keys[i] + 1) % m != keys[i + 1] {
            let tablesum = table.iter().map(|&k| k * cnt[&k]).sum::<usize>();
            ans = ans.min(asum - tablesum);
            table = vec![];
        }
    }

    println!("{}", ans);
}

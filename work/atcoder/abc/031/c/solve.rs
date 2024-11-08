#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use proconio::{marker::*, *};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut sums = vec![vec![[0; 2]; n]; n];

    for i in 0..n - 1 {
        for j in i + 1..n {
            sums[i][j][0] = a[i..=j].iter().step_by(2).sum();
            sums[i][j][1] = a[i..=j].iter().skip(1).step_by(2).sum();
        }
    }

    let mut takahashi_max = i32::MIN;

    for i in 0..n {
        let mut aoki_max = i32::MIN;
        let mut takahashi = 0;
        for j in 0..n {
            if i == j {
                continue;
            } else {
                let aoki = sums[i.min(j)][i.max(j)][1];
                if aoki > aoki_max {
                    // 左から走査しているので真に大きい時のみ更新することで
                    // 「同じ点数なら最も左を選ぶ」という条件を満たす
                    aoki_max = aoki;
                    takahashi = sums[i.min(j)][i.max(j)][0];
                }
            }
        }
        takahashi_max = takahashi_max.max(takahashi);
    }

    println!("{}", takahashi_max);
}

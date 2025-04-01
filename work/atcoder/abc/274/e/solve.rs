#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use core::f64;
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
        xy: [(isize, isize); n],
        pq: [(isize, isize); m],
    }

    // 町と宝箱を一緒に扱う
    let xy = pq.into_iter().chain(xy).collect_vec();

    // dp[s][i]
    // 訪問済みの町/宝箱の集合が s で最後に訪れた町/宝箱 が i のときの最短移動時間
    let mut dp = vec![vec![f64::INFINITY; n + m]; 1 << (n + m)];

    for s in 1..1_usize << (n + m) {
        // 初回の訪問先
        if s.count_ones() == 1 {
            for i in 0..(n + m) {
                if s >> i & 1 == 0 {
                    continue;
                }

                let (xi, yi) = (xy[i].0 as f64, xy[i].1 as f64);

                for i in 0..(n + m) {
                    dp[s][i] = (xi.powi(2) + yi.powi(2)).sqrt();
                }
            }
            continue;
        }

        // 二番目以降の訪問先
        for i in 0..(n + m) {
            if s >> i & 1 == 0 {
                continue;
            }

            let (xi, yi) = (xy[i].0 as f64, xy[i].1 as f64);

            let from = s - (1 << i);
            let booster = (from & ((1 << m) - 1)).count_ones();

            for j in 0..(n + m) {
                if from >> j & 1 == 0 {
                    continue;
                }

                let (xj, yj) = (xy[j].0 as f64, xy[j].1 as f64);

                dp[s][i] = dp[s][i].min(
                    dp[from][j]
                        + ((xi - xj).powi(2) + (yi - yj).powi(2)).sqrt()
                            / (2_usize.pow(booster) as f64),
                );
            }
        }
    }

    let mut ans = f64::INFINITY;

    for i in 0..1_usize << m {
        let from = (((1 << n) - 1) << m) | i;
        let booster = i.count_ones();

        for j in 0..(n + m) {
            let (xj, yj) = (xy[j].0 as f64, xy[j].1 as f64);
            let d = dp[from][j] + (xj.powi(2) + yj.powi(2)).sqrt() / (2_usize.pow(booster) as f64);

            ans = ans.min(d);
        }
    }

    println!("{}", ans);
}

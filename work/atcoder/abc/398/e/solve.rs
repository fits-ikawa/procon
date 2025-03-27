#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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

fn main() {
    input_interactive! {
        n: usize,
        uv: [(Usize1, Usize1); n-1],
    }

    let mut dp = vec![vec![usize::MAX; n]; n];

    for i in 0..n {
        dp[i][i] = 0;
    }

    for (u, v) in uv {
        dp[u][v] = 1;
        dp[v][u] = 1;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(dp[k][j]));
            }
        }
    }

    let mut cand = btreeset! {};

    for i in 0..n - 1 {
        for j in i + 1..n {
            if dp[i][j] >= 3 && dp[i][j] % 2 == 1 {
                cand.insert((i + 1, j + 1));
            }
        }
    }

    let mut turn = cand.len() % 2 == 1;

    println!("{}", if turn { "First" } else { "Second" });

    loop {
        if turn {
            let hand = cand.pop_first().unwrap();
            println!("{} {}", hand.0, hand.1);
        } else {
            input_interactive! {
                mut hand: (isize, isize),
            }

            if hand == (-1, -1) {
                break;
            }

            let hand = if hand.0 < hand.1 {
                (hand.0 as usize, hand.1 as usize)
            } else {
                (hand.1 as usize, hand.0 as usize)
            };

            cand.remove(&hand);
        }
        turn = !turn;
    }
}

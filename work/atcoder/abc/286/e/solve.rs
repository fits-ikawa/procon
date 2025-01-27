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
        n: usize,
        a: [usize; n],
        s: [Chars; n],
        q: usize,
        uv: [(Usize1, Usize1); q],
    }

    // ワーシャルフロイド法
    let mut dp_cost = vec![vec![usize::MAX; n]; n];
    let mut dp_value = vec![vec![usize::MIN; n]; n];

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                dp_cost[i][j] = 1;
                dp_value[i][j] = a[i] + a[j];
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let c = dp_cost[i][k].saturating_add(dp_cost[k][j]);
                let v = match (dp_value[i][k], dp_value[k][j]) {
                    (x, y) if x > 0 && y > 0 => x + y - a[k],
                    _ => 0,
                };

                #[allow(clippy::comparison_chain)]
                if c < dp_cost[i][j] {
                    // 最短コストが更新された場合は上書き
                    dp_cost[i][j] = c;
                    dp_value[i][j] = v;
                } else if c == dp_cost[i][j] {
                    // 最短コストが同じなら高い方の価値を選ぶ
                    dp_value[i][j] = dp_value[i][j].max(v);
                }
            }
        }
    }

    for (u, v) in uv {
        if dp_cost[u][v] == usize::MAX {
            println!("Impossible");
        } else {
            println!("{} {}", dp_cost[u][v], dp_value[u][v]);
        }
    }
}

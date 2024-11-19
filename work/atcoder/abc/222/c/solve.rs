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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [Chars; n*2],
    }

    let mut players = (0..n * 2).map(|i| (0, i)).collect_vec();

    for round in 0..m {
        for k in 0..n {
            let x_rank = 2 * k;
            let y_rank = 2 * k + 1;
            let (x_win, x_i) = players[x_rank];
            let (y_win, y_i) = players[y_rank];
            let x_hand = a[x_i][round];
            let y_hand = a[y_i][round];

            if janken(x_hand, y_hand) {
                players[x_rank] = (x_win - 1, x_i);
            } else if janken(y_hand, x_hand) {
                players[y_rank] = (y_win - 1, y_i);
            }
        }

        // 勝数をマイナスで記録しているので
        // 「勝数の降順、ただし勝数が同じなら番号の昇順」になる
        players.sort();
    }

    println!("{}", players.iter().map(|(_, i)| i + 1).join("\n"));
}

fn janken(a: char, b: char) -> bool {
    // a が勝つかだけ判定
    (a == 'G' && b == 'C') || (a == 'C' && b == 'P') || (a == 'P' && b == 'G')
}

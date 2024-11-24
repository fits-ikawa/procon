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
        n: usize, q: usize,
        s: Chars,
        lr: [(Usize1, Usize1); q],
    }

    // '1' の出現数の累積和
    let mut acc1 = s
        .iter()
        .map(|&c| if c == '1' { 1 } else { 0 })
        .cumsum::<usize>()
        .collect_vec();
    acc1.insert(0, 0);

    // '2' の出現数の累積和
    let mut acc2 = s
        .iter()
        .map(|&c| if c == '2' { 1 } else { 0 })
        .cumsum::<usize>()
        .collect_vec();
    acc2.insert(0, 0);

    // '/' の出現位置（昇順）
    let slash = s
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| if c == '/' { Some(i) } else { None })
        .collect_vec();

    for (l, r) in lr {
        // 最長の 11/22 文字列の '1', '2' の個数を二分探索
        let mut left = 0;
        let mut right = n;

        while right - left > 1 {
            let mid = (left + right) / 2;
            let need = mid; // 内側の二分探索で参照するためコピー

            // '1', '2' がそれぞれ need 個ずつある部分列を作れるか？
            // まず '1' を確保する
            let one_pos = {
                // '1' を必要数確保できる位置（の 1 個右）を二分探索
                let mut left = l;
                let mut right = r + 1;
                while right - left > 1 {
                    let mid = (left + right) / 2;
                    if acc1[mid] - acc1[l] >= need {
                        right = mid;
                    } else {
                        left = mid;
                    }
                }
                right
            };

            if acc1[one_pos] - acc1[l] < need {
                // '1' が足りない
                right = mid;
                continue;
            }

            // '/' を探す
            let slash_i = slash.lower_bound(&one_pos);
            if slash_i == slash.len() || slash[slash_i] > r {
                // '/' がない
                right = mid;
                continue;
            }
            let slash_pos = slash[slash_i];

            // '/' 以降の '2' を数える
            if acc2[r + 1] - acc2[slash_pos + 1] < need {
                // '2' が足りない
                right = mid;
                continue;
            }

            left = mid;
        }

        if left > 0 {
            println!("{}", left * 2 + 1);
        } else {
            // left == 0 でも '/' があるなら長さ 1 の部分列にできる
            let slash_pos = slash.lower_bound(&l);
            if slash_pos == slash.len() {
                println!("0");
            } else {
                // r までに出てくるかもチェック
                println!("{}", if slash[slash_pos] <= r { 1 } else { 0 });
            }
        }
    }
}

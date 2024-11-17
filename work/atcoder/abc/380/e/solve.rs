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
    // BTreeMap で実装
    input! {
        n: usize, q: usize,
    }

    // 色分けされた各領域の左端の番号 -> その領域の色
    let mut area = (0..n).map(|i| (i, i)).collect::<BTreeMap<_, _>>();
    // 色ごとの個数
    let mut count = vec![1; n];

    for _ in 0..q {
        input! {
            k: usize,
        }

        match k {
            1 => {
                input! {
                    x: Usize1, c: Usize1,
                }

                // x を含む範囲 [left, right) を求める
                let (&left, &color) = area.range(..=x).last().unwrap();
                let (&right, _) = area.range(x + 1..).next().unwrap_or((&n, &0));

                // 色とその数の更新
                count[color] -= right - left;
                area.insert(left, c);
                count[c] += right - left;

                // 左に隣接している範囲の色が同じなら統合
                if let Some((_, &lc)) = area.range(..left).last() {
                    if lc == c {
                        area.remove(&left);
                    }
                }

                // 右も同様
                if let Some(&rc) = area.get(&right) {
                    if c == rc {
                        area.remove(&right);
                    }
                }
            }
            2 => {
                input! {
                    c: Usize1
                }

                println!("{}", count[c]);
            }
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
fn solve() {
    // BTreeSet + Vec で実装
    input! {
        n: usize, q: usize,
    }

    // 色の境界（左端の番号）のリスト
    let mut fences = (0..n).collect::<BTreeSet<_>>();

    let mut f2color = (0..n).collect_vec();
    let mut color_cnt = vec![1; n];

    for _ in 0..q {
        input! {
            k: usize,
        }

        match k {
            1 => {
                input! {
                    x: Usize1, c: Usize1,
                }

                // x を含む範囲 [left, right) を求める
                let left = fences.range(..=x).next_back().copied().unwrap();
                let right = fences.range(x + 1..).next().copied().unwrap_or(n);

                // 色数の更新
                color_cnt[f2color[left]] -= right - left;
                f2color[left] = c;
                color_cnt[f2color[left]] += right - left;

                // 左に隣接している範囲の色が同じなら統合
                if let Some(&l) = fences.range(..left).next_back() {
                    if f2color[l] == f2color[left] {
                        // 境界を消すことで統合とする
                        fences.remove(&left);
                    }
                }

                // 右も同様
                if let Some(&r) = fences.get(&right) {
                    if f2color[left] == f2color[r] {
                        fences.remove(&r);
                    }
                }
            }
            2 => {
                input! {
                    c: Usize1
                }

                println!("{}", color_cnt[c]);
            }
            _ => unreachable!(),
        }
    }
}

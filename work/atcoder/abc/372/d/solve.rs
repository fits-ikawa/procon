#![allow(unused_imports)]
use ac_library::{Max, Segtree};
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
        n: usize,
        h: [usize; n],
    }

    let mut b = vec![h[n - 1]];
    let mut ans = vec![0; n];

    for i in (0..n - 1).rev() {
        ans[i] = b.len();
        while !b.is_empty() && *b.last().unwrap() < h[i] {
            b.pop();
        }
        b.push(h[i]);
    }

    println!("{}", ans.iter().join(" "));
}

#[allow(dead_code)]
fn solve() {
    // セグ木で何とかしようと頑張って TLE した解法
    input! {
        n: usize,
        h: [usize; n],
    }

    let st = Segtree::<Max<_>>::from(h);

    let ans = (0..n)
        .map(|i| {
            if i == n - 1 {
                return 0;
            }
            let mut j = i + 1;
            let mut valid_j = 1;
            loop {
                // h が 1, 2, 3, ... のような昇順で並んでたら
                // N 回ループするので O(N^2) になってしまう。
                // （そうじゃないとしても高々定数倍の違い）
                // また、i が動いても j が同じならそこから右への検索が
                // 全く同じ処理の繰り返しになってしまう、という無駄に気づけていれば
                // 公式解説の解法に向かえたかも
                j = st.max_right(j, |&x| x <= st.get(j));
                if j == n {
                    break;
                }
                valid_j += 1;
            }
            valid_j
        })
        .collect_vec();

    println!("{}", ans.iter().join(" "));
}

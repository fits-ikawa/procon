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

#[fastout]
fn main() {
    input! {
        n: usize, mut k: usize,
        mut a: [usize; n],
    }

    // 二分探索

    let mut left = 0;
    let mut right = 2000000000000;

    while right - left > 1 {
        let mid = (left + right) / 2;

        if a.iter().map(|&ai| ai.min(mid)).sum::<usize>() <= k {
            left = mid;
        } else {
            right = mid;
        }
    }

    // left は k 個以下食べるときの最大周回数なので k に足りていないケースがある。
    // それを次にシミュレートする

    let mut ans = a.iter().map(|ai| ai.saturating_sub(left)).collect_vec();
    let mut remain = k - (a.iter().sum::<usize>() - ans.iter().sum::<usize>());

    // 次の 1 周以内に必ず k 個に達する
    for i in 0..n {
        if ans[i] > 0 && remain > 0 {
            ans[i] -= 1;
            remain -= 1;
        }
    }

    println!("{}", ans.iter().join(" "));
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, mut k: usize,
        mut a: [usize; n],
    }

    // シミュレーション

    let mut baskets = (0..n).filter(|&i| a[i] > 0).collect::<BTreeSet<_>>();
    let mut cur = baskets.first().copied().unwrap();

    while k > 0 {
        let avg = k / baskets.len();
        let rem = k % baskets.len();
        let b = baskets.iter().copied().chain(baskets.clone()).collect_vec();
        let pos = b.iter().position(|&i| i == cur).unwrap();
        let add = b[pos..(pos + rem)].iter().copied().collect::<HashSet<_>>();
        let next_cur = b[pos + rem];

        for i in baskets.clone() {
            let eat = (avg + if add.contains(&i) { 1 } else { 0 }).min(a[i]);
            k -= eat;
            a[i] -= eat;

            if a[i] == 0 {
                baskets.remove(&i);
            }
        }

        if baskets.is_empty() {
            break;
        }

        cur = if let Some(next) = baskets.range(next_cur..).next().copied() {
            next
        } else {
            baskets.first().copied().unwrap()
        };
    }

    println!("{}", a.iter().join(" "));
}

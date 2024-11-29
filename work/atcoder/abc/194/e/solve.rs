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
    // 計算量が O(NlogN) だとちゃんと言える版
    input! {
        n: usize, m: usize,
        a: [usize; n],
    }

    let mut count = vec![0; n + 1];
    let mut mex_stock = btreeset! {};

    for &ai in &a[0..m] {
        count[ai] += 1;
    }

    for i in 0..=n {
        if count[i] == 0 {
            mex_stock.insert(i);
        }
    }

    let mut ans = *mex_stock.first().unwrap();

    for i in 1..n - m + 1 {
        let go = a[i - 1];
        let come = a[i + m - 1];

        count[go] -= 1;
        if count[go] == 0 {
            mex_stock.insert(go);
        }

        count[come] += 1;
        if count[come] == 1 {
            mex_stock.remove(&come);
        }

        ans = ans.min(*mex_stock.first().unwrap());
    }

    println!("{}", ans);
}

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
fn solve2() {
    // 解説 AC
    // O(N) のはずだが、なんか遅い……（Vec の確保とか？）
    input! {
        n: usize, m: usize,
        a: [usize; n],
    }

    let mut pos = vec![vec![0]; n + 1];

    for i in 0..n {
        pos[a[i]].push(i + 1);
    }

    for i in 0..=n {
        pos[i].push(n + 1);
    }

    for i in 0..=n {
        for j in 0..pos[i].len() - 1 {
            if pos[i][j + 1] - pos[i][j] > m {
                println!("{}", i);
                return;
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
fn solve1() {
    // ガチャガチャやってなんとかなった版
    input! {
        n: usize, m: usize,
        a: [usize; n],
    }

    let mut nums = btreemap! {};

    for i in 0..m {
        *nums.entry(a[i]).or_insert(0) += 1;
    }

    let mut cur = 0;

    for &k in nums.keys() {
        if k != cur {
            break;
        }
        cur = k + 1;
    }

    let mut ans = cur;

    for i in 1..n - m + 1 {
        let mut next = cur;

        if nums[&a[i - 1]] == 1 {
            // 出ていった数が現在の mex より小さければ更新
            nums.remove(&a[i - 1]);
            next = cur.min(a[i - 1]);
        } else {
            *nums.get_mut(&a[i - 1]).unwrap() -= 1;
        }

        *nums.entry(a[i + m - 1]).or_insert(0) += 1;
        if nums[&a[i + m - 1]] == 1 && next == a[i + m - 1] {
            // 入ってきた数が現在の mex と同じなら
            // 次に空いている数を探す
            if a[i - 1] == a[i + m - 1] {
                // 出ていったのと同じ数が入ってきたなら元に戻す
                next = cur;
            } else {
                // range は O(logN) だが新しい mex が見つかるまでの
                // 平均計算量がわからない（AC はできた）
                for (&k, _) in nums.range(next..) {
                    if k != next {
                        break;
                    }
                    next = k + 1;
                }
            }
        }
        cur = next;
        ans = ans.min(cur);
    }

    println!("{}", ans);
}

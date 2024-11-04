use proconio::input;
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};

#[allow(unused_imports)]
use std::cmp::{max, min};

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize, w: usize,
        items: [(usize, usize); n],
    }

    let v = items.iter().map(|(_, v)| v).sum::<usize>();

    let mut dp = vec![None; v + 1];
    dp[0] = Some(0);

    for &(item_w, item_v) in &items {
        for i in (0..=v - item_v).rev() {
            if let Some(prev_w) = dp[i] {
                dp[i + item_v] = Some(dp[i + item_v].unwrap_or(usize::MAX).min(prev_w + item_w));
            }
        }
    }

    println!(
        "{}",
        (0..=v)
            .filter(|&i| dp[i].unwrap_or(w + 1) <= w)
            .max()
            .unwrap()
    );
}

#[allow(dead_code)]
fn solve2() {
    input! {
        n: usize, w: usize,
        items: [(usize, usize); n],
    }

    let v = items.iter().map(|(_, v)| v).sum::<usize>();

    let mut dp = vec![vec![None; v + 1]; n + 1];
    dp[0][0] = Some(0);

    for i in 1..=n {
        let (item_w, item_v) = items[i - 1];
        for j in 0..=v {
            dp[i][j] = dp[i - 1][j];

            if j >= item_v {
                if let Some(prev_w) = dp[i - 1][j - item_v] {
                    // dp[i][j] = match dp[i][j] {
                    //     Some(cur_w) => Some(min(cur_w, prev_w + item_w)),
                    //     None => Some(prev_w + item_w),
                    // }
                    dp[i][j] = Some(dp[i][j].unwrap_or(prev_w + item_w).min(prev_w + item_w));
                }
            }
        }
    }

    let mut picked = Vec::new();
    for j in 0..=v {
        if let Some(weight) = dp[n][j] {
            if weight <= w {
                picked.push(j);
            }
        }
    }

    println!("{}", picked.iter().max().unwrap());
}

#[allow(dead_code)]
fn solve1() {
    input! {
        n: usize, w: usize,
        items: [(usize, usize); n],
    }

    let v = items.iter().map(|(_, v)| v).sum::<usize>();

    let mut dp = vec![vec![usize::MAX; v + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        let (item_w, item_v) = items[i - 1];
        for j in 0..item_v {
            dp[i][j] = dp[i - 1][j];
        }
        for j in item_v..=v {
            dp[i][j] = min(
                dp[i - 1][j],
                dp[i - 1][j - item_v]
                    .checked_add(item_w)
                    .unwrap_or(usize::MAX),
            );
        }
    }

    let mut picked = Vec::new();
    for j in 0..=v {
        if dp[n][j] <= w {
            picked.push(j);
        }
    }

    println!("{}", picked.iter().max().unwrap());
}

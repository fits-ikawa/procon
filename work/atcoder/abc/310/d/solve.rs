#![allow(clippy::comparison_chain)]
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
        n: usize, t: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut dislike = vec![vec![false; n]; n];

    for (a, b) in ab {
        dislike[a][b] = true;
        dislike[b][a] = true;
    }

    let mut ans = 0;

    dfs(0, &mut vec![], &mut ans, &dislike, n, t);

    println!("{}", ans);
}

fn dfs(
    i: usize,
    groups: &mut Vec<Vec<usize>>,
    ans: &mut usize,
    dislike: &[Vec<bool>],
    n: usize,
    t: usize,
) {
    if i == n {
        // ベル数の分け方から t 個のグループを作れたものだけ判定
        if groups.len() == t
            && groups
                .iter()
                .all(|g| g.iter().combinations(2).all(|c| !dislike[*c[0]][*c[1]]))
        {
            *ans += 1;
        }
        return;
    }

    // 既存のグループから選ぶ
    for j in 0..groups.len() {
        groups[j].push(i);
        dfs(i + 1, groups, ans, dislike, n, t);
        groups[j].pop();
    }

    // 新しくグループを作る
    groups.push(vec![i]);
    dfs(i + 1, groups, ans, dislike, n, t);
    groups.pop();
}

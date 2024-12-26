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

use ac_library::ModInt1000000007 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [char; n],
        ab: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    // dp[i][j]
    // 頂点 0 を根とした木における、頂点 i を根とした部分木の辺を削除していくつかの連結成分に分解するとき
    // i を含む連結成分が j（0: a だけ含む, 1: b だけ含む, 2: 両方含む）であるときの通り数。
    // いずれの場合も、他の（i を含まない）連結成分は a, b 両方を含むという条件を満たしたうえでの通り数である
    let mut dp = vec![vec![Mint::new(0); 3]; n];

    dfs(0, usize::MAX, &mut dp, &adj, &c);

    println!("{}", dp[0][2]);
}

fn dfs(v: usize, p: usize, dp: &mut [Vec<Mint>], adj: &[Vec<usize>], c: &[char]) {
    let (me, other) = if c[v] == 'a' { (0, 1) } else { (1, 0) };

    dp[v][me] = Mint::new(1);
    dp[v][2] = Mint::new(1); // 葉のとき後で 0 になるようつじつま合わせ

    for &w in &adj[v] {
        if w != p {
            dfs(w, v, dp, adj, c);

            // dp[v][me]: v を含む連結成分が a/b だけ含む場合というのは
            // dp[w][me]: a/b だけの子を繋いだ場合と
            // dp[w][2]: a, b 両方含む子を切り離した場合を足したもの
            // ひとつの子について上記で、それを子がある分だけ掛け合わせる
            dp[v][me] = dp[v][me] * (dp[w][me] + dp[w][2]);

            // dp[v][2]: v を含む連結成分が a, b 両方含む場合というのは
            // dp[w][me]: a/b だけの子を繋いだ場合（★1）と
            // dp[w][other]: b/a だけの子を繋いだ場合と
            // dp[w][2] * 2: a, b 両方含む子を繋いだ場合と、切り離した場合（★2）を足したもの
            // ひとつの子について上記で、それを子がある分だけ掛け合わせる。
            // ただしこの通り数には、子がすべて ★1、★2 を選択して a/b だけになってしまうケースも含まれる（続く）
            dp[v][2] = dp[v][2] * (dp[w][me] + dp[w][other] + dp[w][2] * 2);
        }
    }

    // （続き）それを最後に引く
    dp[v][2] = dp[v][2] - dp[v][me];
}

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
        n: usize, m: usize, k: isize,
        a: [Usize1; m],
        uv: [(Usize1, Usize1); n-1],
    }

    let mut cnt = hashmap! {};
    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);

        if u <= v {
            cnt.insert((u, v), 0);
        } else {
            cnt.insert((v, u), 0);
        }
    }

    for i in 0..m - 1 {
        dfs(a[i], usize::MAX, a[i + 1], &mut vec![], &mut cnt, &adj);
    }

    let b = cnt.values().copied().collect_vec();
    let sum = b.iter().sum::<isize>();

    if (sum + k) % 2 != 0 {
        println!("0");
        return;
    }

    use ac_library::ModInt998244353 as Mint;

    let mx = ((sum + k.abs()) / 2) as usize;

    let mut dp = vec![Mint::new(0); mx + 1];
    dp[0] = Mint::new(1);

    for i in 1..=n - 1 {
        let bi = b[i - 1] as usize;
        let mut next_dp = vec![Mint::new(0); mx + 1];

        for j in 0..=mx {
            next_dp[j] += dp[j];
            if j + bi <= mx {
                next_dp[j + bi] += dp[j];
            }
        }

        dp = next_dp;
    }

    println!("{}", dp[mx]);
}

fn dfs(
    v: usize,
    p: usize,
    goal: usize,
    path: &mut Vec<(usize, usize)>,
    cnt: &mut HashMap<(usize, usize), isize>,
    adj: &[Vec<usize>],
) -> bool {
    if v == goal {
        for (s, t) in path {
            if s <= t {
                cnt.entry((*s, *t)).and_modify(|e| *e += 1);
            } else {
                cnt.entry((*t, *s)).and_modify(|e| *e += 1);
            }
        }
        return true;
    }

    for &w in &adj[v] {
        if w != p {
            path.push((v, w));
            if dfs(w, v, goal, path, cnt, adj) {
                return true;
            }
            path.pop();
        }
    }

    false
}

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
use petgraph::graph::edge_index;
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
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut uf = ac_library::Dsu::new(n);

    for &(u, v) in &uv {
        uf.merge(u, v);
    }

    let mut edge_cnt = vec![0; n];

    for &(u, _) in &uv {
        edge_cnt[uf.leader(u)] += 1;
    }

    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    use ac_library::ModInt998244353 as Mint;

    let mut ans = Mint::new(1);

    for g in uf.groups() {
        if g.len() <= 1 || g.len() != edge_cnt[uf.leader(g[0])] {
            println!("0");
            return;
        }

        ans *= 2;
    }

    println!("{}", ans);
}

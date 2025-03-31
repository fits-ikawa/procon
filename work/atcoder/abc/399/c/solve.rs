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
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut cnt = vec![0; n];

    for (u, v) in uv {
        if uf.same(u, v) {
            cnt[uf.leader(u)] += 1;
        } else {
            let cu = cnt[uf.leader(u)];
            let cv = cnt[uf.leader(v)];

            let newl = uf.merge(u, v);
            cnt[newl] = cu + cv + 1;
        }
    }

    let mut ans = 0;

    for g in uf.groups() {
        ans += cnt[uf.leader(g[0])] - (g.len() - 1);
    }

    println!("{}", ans);
}

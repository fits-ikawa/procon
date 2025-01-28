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
        k: usize,
        xy: [(Usize1, Usize1); k],
        q: usize,
        pq: [(Usize1, Usize1); q],
    }

    let mut uf = ac_library::Dsu::new(n);

    for (u, v) in uv {
        uf.merge(u, v);
    }

    let pairs = xy
        .into_iter()
        .flat_map(|(x, y)| {
            let (lx, ly) = (uf.leader(x), uf.leader(y));
            [(lx, ly), (ly, lx)]
        })
        .collect::<HashSet<_>>();

    for (p, q) in pq {
        let (lp, lq) = (uf.leader(p), uf.leader(q));
        println!("{}", if pairs.contains(&(lp, lq)) {"No"} else {"Yes"});
    }
}

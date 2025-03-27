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
        n: usize, m: usize, q: usize,
        abc: [(Usize1, Usize1, usize); m],
        uvw: [(Usize1, Usize1, usize); q],
    }

    let mut todo = BinaryHeap::new();

    for (a, b, c) in abc {
        todo.push((Reverse(c), None, a, b));
    }

    for (i, &(u, v, w)) in uvw.iter().enumerate() {
        todo.push((Reverse(w), Some(i), u, v));
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut ans = vec![false; q];

    while let Some((Reverse(_), query, a, b)) = todo.pop() {
        if let Some(i) = query {
            ans[i] = !uf.same(a, b);
        } else {
            uf.merge(a, b);
        }
    }

    println!(
        "{}",
        ans.iter().map(|&x| if x { "Yes" } else { "No" }).join("\n")
    );
}

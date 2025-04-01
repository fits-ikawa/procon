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
        n: usize, m: usize, k: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); k],
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut friend = vec![vec![]; n];
    let mut block = vec![vec![]; n];

    for (a, b) in ab {
        uf.merge(a, b);
        friend[a].push(b);
        friend[b].push(a);
    }

    for (c, d) in cd {
        block[c].push(d);
        block[d].push(c);
    }

    let mut ans = vec![0; n];

    for i in 0..n {
        ans[i] = uf.size(i) - friend[i].len() - 1;

        for &b in &block[i] {
            if uf.same(i, b) {
                ans[i] -= 1;
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}

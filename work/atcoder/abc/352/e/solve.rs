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
    }

    let mut k = vec![];
    let mut c = vec![];
    let mut a = vec![];

    for _ in 0..m {
        input! {
            ki: usize, ci: usize,
            ai: [Usize1; ki],
        }

        k.push(ki);
        c.push(ci);
        a.push(ai);
    }

    let mut edges = btreeset! {};

    for i in 0..m {
        for j in 1..k[i] {
            edges.insert((c[i], a[i][0], a[i][j]));
        }
    }

    // クラスカル法
    let mut uf = ac_library::Dsu::new(n);
    let mut ans = 0;

    for (c, u, v) in edges {
        if !uf.same(u, v) {
            uf.merge(u, v);
            ans += c;
        }
    }

    if uf.groups().len() == 1 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

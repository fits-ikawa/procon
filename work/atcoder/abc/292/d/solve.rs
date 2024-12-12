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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut extras = vec![];

    for (u, v) in uv {
        if uf.same(u, v) {
            extras.push(u);
        }
        uf.merge(u, v);
    }

    let extras = extras.into_iter().map(|u| uf.leader(u)).counts();

    if uf.groups().len() == extras.len() && extras.values().all(|&v| v == 1) {
        println!("Yes");
    } else {
        println!("No");
    }

    let mut multiset = hashmap! {};
    multiset.insert(1, 1);
    if let Some(value) = multiset.get_mut(&1) {
        *value -= 1;
        if *value == 0 {
            multiset.remove(&1);
        }
    }

    debug!(multiset);
}

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
        n: usize,
        st: [(String, String); n],
    }

    let names = st
        .iter()
        .map(|(s, _)| s)
        .chain(st.iter().map(|(_, t)| t))
        .unique()
        .collect_vec();

    let n2i = names
        .iter()
        .enumerate()
        .map(|(i, &name)| (name, i))
        .collect::<HashMap<_, _>>();

    let mut uf = ac_library::Dsu::new(names.len());

    for (s, t) in &st {
        if uf.same(n2i[s], n2i[t]) {
            println!("No");
            return;
        }
        uf.merge(n2i[s], n2i[t]);
    }

    println!("Yes");
}

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
        ab: [(Usize1, Usize1); m],
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut excables = vec![];

    for (i, &(a, b)) in ab.iter().enumerate() {
        if uf.same(a, b) {
            excables.push((a, b, i));
        } else {
            uf.merge(a, b);
        }
    }

    let leaders = (0..n).map(|i| uf.leader(i)).unique().collect_vec();

    if leaders.len() == 1 {
        println!("0");
        return;
    }

    let mut cables = vec![vec![]; n];

    for c in excables {
        cables[uf.leader(c.0)].push(c);
    }

    let leaders = leaders
        .into_iter()
        .sorted_by_key(|&i| Reverse(cables[i].len()))
        .collect_vec();

    let mut from = 0;
    let mut ans = vec![];

    for to in 1..leaders.len() {
        if cables[leaders[from]].is_empty() {
            from += 1;
        }

        let (a, _, ci) = cables[leaders[from]].pop().unwrap();

        ans.push((ci + 1, a + 1, leaders[to] + 1));
    }

    println!("{}", ans.len());

    if !ans.is_empty() {
        for (i, a, b) in ans {
            println!("{} {} {}", i, a, b);
        }
    }
}

#![allow(unused_imports)]
use ac_library::Dsu;
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
        ab: [(Usize1, Usize1); m],
    }

    let mut uf = Dsu::new(n);

    for &(a, b) in &ab {
        uf.merge(a, b);
    }

    let mut edges = vec![0; n];

    for (a, _) in ab {
        let leader = uf.leader(a);
        edges[leader] += 1;
    }

    let mut ans = 0;
    for group in uf.groups() {
        let len = group.len();
        let leader = uf.leader(group[0]);
        ans += len * (len - 1) / 2 - edges[leader];
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }

    use petgraph::unionfind::UnionFind;
    let mut uf = UnionFind::new(n);

    for &(a, b) in &ab {
        uf.union(a, b);
    }

    let mut edges = hashmap! {};
    let mut vertices: HashMap<_, HashSet<_>> = hashmap! {};

    for (a, b) in ab {
        let parent = uf.find(a);
        edges.entry(parent).and_modify(|e| *e += 1).or_insert(1);
        vertices
            .entry(parent)
            .and_modify(|e| {
                (*e).insert(a);
                (*e).insert(b);
            })
            .or_insert(hashset! {a, b});
    }

    let mut ans = 0;
    for parent in edges.keys() {
        let len = vertices[parent].len();
        ans += len * (len - 1) / 2 - edges[parent];
    }

    println!("{}", ans);
}

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
        n: usize,
        sx: isize, sy: isize, tx: isize, ty: isize,
        xyr: [(isize, isize, isize); n],
    }

    let mut adj = vec![vec![]; n];

    // 円周が重なる・交差する円同士を辺で繋ぐ
    for i in 0..n - 1 {
        let (ix, iy, ir) = xyr[i];
        for j in i + 1..n {
            let (jx, jy, jr) = xyr[j];
            let d = distsq((ix, iy), (jx, jy));
            let inner = (ir - jr).pow(2) <= d;
            let outer = d <= (ir + jr).pow(2);
            if inner && outer {
                adj[i].push(j);
                adj[j].push(i);
            }
        }
    }

    // スタートの円周、ゴールの円周を調べる
    let mut start = 0;
    let mut goal = 0;

    for i in 0..n {
        let (ix, iy, ir) = xyr[i];
        if distsq((sx, sy), (ix, iy)) == ir.pow(2) {
            start = i;
        }
        if distsq((tx, ty), (ix, iy)) == ir.pow(2) {
            goal = i;
        }
    }

    // BFS で到達可能性を調べる
    let mut todo = VecDeque::new();
    let mut seen = vec![false; n];
    todo.push_back(start);
    seen[start] = true;

    while let Some(from) = todo.pop_front() {
        if from == goal {
            println!("Yes");
            return;
        }

        for &to in &adj[from] {
            if !seen[to] {
                seen[to] = true;
                todo.push_back(to);
            }
        }
    }

    println!("No");
}

fn distsq(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

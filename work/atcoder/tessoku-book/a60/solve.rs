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
        n: usize,
        a: [usize; n],
    }

    // 解説 AC

    let mut stack: Vec<(usize, isize)> = Vec::with_capacity(n);
    let mut ans = Vec::with_capacity(n);

    for i in 0..n {
        while !stack.is_empty() && stack.last().copied().unwrap().0 < a[i] {
            stack.pop();
        }

        ans.push(if stack.is_empty() {
            -1
        } else {
            stack.last().copied().unwrap().1
        });

        stack.push((a[i], (i + 1) as isize));
    }

    println!("{}", ans.iter().join(" "));
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // セグ木解法

    let map = a
        .iter()
        .sorted()
        .copied()
        .enumerate()
        .map(|(i, ai)| (ai, i))
        .collect::<HashMap<_, _>>();

    let a_comp = a.iter().map(|ai| map[ai]).collect_vec();

    use ac_library::{Max, Segtree};
    let mut seg = Segtree::<Max<usize>>::new(n);

    let mut ans = Vec::with_capacity(n);

    for i in 0..n {
        let pos = seg.prod(a_comp[i]..);
        ans.push(if pos > 0 { pos as isize } else { -1 });

        seg.set(a_comp[i], i + 1);
    }

    println!("{}", ans.iter().join(" "));
}

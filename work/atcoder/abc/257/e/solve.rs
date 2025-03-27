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
        c: [usize; 9],
    }

    let c = c
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .sorted_by(|a, b| match a.1.cmp(&b.1) {
            Equal => b.0.cmp(&a.0),
            other => other,
        })
        .collect_vec();

    let mut m = n;
    let mut ans = vec![];

    while c[0].1 <= m {
        ans.push(c[0].0);
        m -= c[0].1;
    }

    if ans.is_empty() {
        println!("0");
        return;
    }

    let mut d = c
        .iter()
        .filter(|x| x.0 > c[0].0 && x.1 > c[0].1)
        .copied()
        .sorted_by_key(|x| x.0)
        .collect_vec();

    let mut i = 0;

    while let Some((num, cost)) = d.pop() {
        let diff = cost - c[0].1;
        while i < ans.len() && diff <= m {
            ans[i] = num;
            m -= diff;
            i += 1;
        }
    }

    println!("{}", ans.iter().join(""));
}

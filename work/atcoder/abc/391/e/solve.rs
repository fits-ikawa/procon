#![allow(clippy::comparison_chain)]
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
        a: Chars,
    }

    let a = a
        .iter()
        .map(|&ai| if ai == '0' { 0 } else { 1 })
        .collect_vec();

    let (_, ans) = dfs(0, 0, n, &a);

    println!("{}", ans);
}

fn dfs(depth: usize, order: usize, n: usize, a: &[usize]) -> (usize, usize) {
    if depth == n - 1 {
        let cnt1 = a[3 * order..3 * order + 3].iter().sum::<usize>();

        return if cnt1 >= 2 {
            (1, cnt1 - 1)
        } else {
            (0, 2 - cnt1)
        };
    }

    let child = (0..3)
        .map(|i| dfs(depth + 1, 3 * order + i, n, a))
        .collect_vec();
    let cnt1 = child.iter().map(|(char, _)| char).sum::<usize>();

    return if cnt1 >= 2 {
        let change = child
            .iter()
            .filter_map(|&(char, cnt)| if char == 1 { Some(cnt) } else { None })
            .sorted()
            .take(cnt1 - 1)
            .sum::<usize>();
        (1, change)
    } else {
        let change = child
            .iter()
            .filter_map(|&(char, cnt)| if char == 0 { Some(cnt) } else { None })
            .sorted()
            .take(2 - cnt1)
            .sum::<usize>();
        (0, change)
    };
}

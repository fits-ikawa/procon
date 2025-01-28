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
        mut a: [usize; n],
    }

    let mut ans = hashset! {};

    dfs(0, &mut vec![], &mut ans, n, &a);

    println!("{}", ans.len());
}

fn dfs(i: usize, groups: &mut Vec<Vec<usize>>, ans: &mut HashSet<usize>, n: usize, a: &[usize]) {
    if i == n {
        let xor = groups
            .iter()
            .map(|g| g.iter().map(|&j| a[j]).sum::<usize>())
            .fold(0, |acc, x| acc ^ x);
        ans.insert(xor);

        return;
    }

    // 既存のグループから選ぶ
    for j in 0..groups.len() {
        groups[j].push(i);
        dfs(i + 1, groups, ans, n, a);
        groups[j].pop();
    }

    // 新しくグループを作る
    groups.push(vec![i]);
    dfs(i + 1, groups, ans, n, a);
    groups.pop();
}

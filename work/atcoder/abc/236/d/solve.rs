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
    }

    let mut a = vec![];

    for i in 1..2 * n {
        input! {
            ai: [usize; 2*n-i],
        }
        a.push(ai);
    }

    let mut people = (0..2 * n).collect::<BTreeSet<_>>();
    let mut ans = 0;

    dfs(&mut people, &mut vec![], &mut ans, n, &a);

    println!("{}", ans);
}

fn dfs(
    people: &mut BTreeSet<usize>,
    pairs: &mut Vec<usize>,
    ans: &mut usize,
    n: usize,
    a: &[Vec<usize>],
) {
    if pairs.len() == n {
        *ans = (*ans).max(pairs.iter().fold(0, |acc, &p| acc ^ p));
        return;
    }

    let i = people.pop_first().unwrap();
    let js = people.iter().copied().collect_vec();
    for j in js {
        pairs.push(a[i][j - (i + 1)]);
        people.remove(&j);
        dfs(people, pairs, ans, n, a);
        people.insert(j);
        pairs.pop();
    }
    people.insert(i);
}

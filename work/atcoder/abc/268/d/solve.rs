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
        n: usize, m: usize,
        s: [String; n],
        t: [String; m],
    }

    let t = t.into_iter().collect::<HashSet<_>>();
    let min_len = s.iter().map(|si| si.len()).sum::<usize>() + s.len() - 1;

    if !(3..=16).contains(&min_len) {
        println!("-1");
        return;
    }

    if n == 1 {
        if t.contains(&s[0]) {
            println!("-1");
        } else {
            println!("{}", s[0]);
        }
        return;
    }

    for perm in s.into_iter().permutations(n) {
        let mut bar = vec![1; n - 1];
        let mut bar_comb = vec![];

        rec(0, 16 - min_len, &mut bar, &mut bar_comb, n);

        for comb in bar_comb {
            let mut username = String::new();
            for i in 0..n - 1 {
                username = username + &perm[i];
                for _ in 0..comb[i] {
                    username.push('_');
                }
            }
            username += perm.last().unwrap();

            if !t.contains(&username) {
                println!("{}", username);
                return;
            }
        }
    }

    println!("-1");
}

fn rec(i: usize, stock: usize, bar: &mut Vec<usize>, bar_comb: &mut Vec<Vec<usize>>, n: usize) {
    if i == n - 1 {
        bar_comb.push(bar.clone());
        return;
    }

    for j in 0..=stock {
        bar[i] += j;
        rec(i + 1, stock - j, bar, bar_comb, n);
        bar[i] -= j;
    }
}

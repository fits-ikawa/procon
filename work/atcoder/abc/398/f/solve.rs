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
        s: Chars,
    }

    // Z アルゴリズム

    let t = s
        .clone()
        .into_iter()
        .rev()
        .chain(s.clone())
        .collect::<String>();

    let z = ac_library::z_algorithm(&t);

    let n = s.len();

    for i in n..n * 2 {
        let l = i - n;
        if z[i] == n - l {
            println!(
                "{}{}{}",
                s[..l].iter().join(""),
                s[l..n].iter().join(""),
                s[..l].iter().rev().join("")
            );
            return;
        }
    }
}

#[allow(dead_code)]
fn solve() {
    input! {
        s: Chars,
    }

    // Manacher のアルゴリズム

    let lr = manacher(&s, '%')
        .into_iter()
        .filter(|&(_, r)| r == s.len())
        .sorted()
        .collect_vec();

    if lr.is_empty() {
        println!(
            "{}{}",
            s.iter().join(""),
            s[..s.len() - 1].iter().rev().join("")
        );
    } else {
        let (l, r) = lr[0];
        println!(
            "{}{}{}",
            s[..l].iter().join(""),
            s[l..r].iter().join(""),
            s[..l].iter().rev().join("")
        );
    }
}

// 文字列 s 中に含まれる回文を検出し、半開区間のリストとして返す
// 自明な（1 文字の）回文は含めず、中心が同じ回文は最長のものの区間が返る
#[allow(dead_code)]
fn manacher(s: &[char], dummy: char) -> Vec<(usize, usize)> {
    let mut t = Vec::with_capacity(s.len() * 2 + 1);
    for &ch in s {
        t.push(dummy);
        t.push(ch);
    }
    t.push(dummy);

    let n = t.len();
    let mut r = vec![0; n];
    let mut i = 0;
    let mut j = 0;

    while i < n {
        while i >= j && i + j < n && t[i - j] == t[i + j] {
            j += 1;
        }

        r[i] = j;

        let mut k = 1;

        while i >= k && k + r[i - k] < j {
            r[i + k] = r[i - k];
            k += 1;
        }

        i += k;
        j -= k;
    }

    let mut ret = vec![];

    for i in 0..n {
        if r[i] <= 2 {
            continue;
        } else {
            let center = i / 2;
            ret.push((
                center - r[i] / 2 + if t[i] == dummy { 0 } else { 1 },
                center + r[i] / 2,
            ));
        }
    }

    ret
}

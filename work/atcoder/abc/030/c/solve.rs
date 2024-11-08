#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use proconio::{marker::*, *};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize, m: usize,
        x: usize, y: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.reverse();
    b.reverse();

    let mut airport_a = true;
    let mut time = 0;
    let mut laps = 0;

    'outer: loop {
        let table = if airport_a { &mut a } else { &mut b };

        loop {
            if let Some(t) = table.pop() {
                if t >= time {
                    time = t + if airport_a { x } else { y };
                    airport_a = !airport_a;
                    if airport_a {
                        laps += 1;
                    }
                    break;
                }
            } else {
                break 'outer;
            }
        }
    }

    println!("{}", laps);
}

#[allow(dead_code)]
fn solve() {
    // 二分探索バージョン（逆に遅かった……）
    input! {
        n: usize, m: usize,
        x: usize, y: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    let mut airport_a = true;
    let mut time = 0;
    let mut laps = 0;

    loop {
        let table = if airport_a { &mut a } else { &mut b };

        let i = table.lower_bound(&time);
        if i != table.len() {
            if table[i] >= time {
                time = table[i] + if airport_a { x } else { y };
                airport_a = !airport_a;
                if airport_a {
                    laps += 1;
                }
            }
        } else {
            break;
        }
    }

    println!("{}", laps);
}

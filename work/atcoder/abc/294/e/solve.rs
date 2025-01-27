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
        l: usize, n1: usize, n2: usize,
        mut vl1: [(usize, usize); n1],
        mut vl2: [(usize, usize); n2],
    }

    let mut events = BinaryHeap::new();

    let mut acc = vl1[0].1;

    for i in 1..n1 {
        events.push(Reverse((acc, true, vl1[i].0)));
        acc += vl1[i].1;
    }
    events.push(Reverse((l, true, usize::MAX)));

    let mut acc = vl2[0].1;

    for i in 1..n2 {
        events.push(Reverse((acc, false, vl2[i].0)));
        acc += vl2[i].1;
    }
    events.push(Reverse((l, false, usize::MAX)));

    let mut m1 = vl1[0].0;
    let mut m2 = vl2[0].0;
    let mut cur = 0;
    let mut ans = 0;

    while let Some(Reverse((pos, is_top, val))) = events.pop() {
        if m1 == m2 {
            ans += pos - cur;
        }

        cur = pos;

        if is_top {
            m1 = val;
        } else {
            m2 = val;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        l: usize, n1: usize, n2: usize,
        mut vl1: [(usize, usize); n1],
        mut vl2: [(usize, usize); n2],
    }

    vl1.push((usize::MAX, 0));
    vl2.push((usize::MAX, 0));

    let mut acc1 = vec![(0, 0); n1];
    let mut acc2 = vec![(0, 0); n2];

    acc1[0] = (vl1[1].0, vl1[0].1);
    acc2[0] = (vl2[1].0, vl2[0].1);

    for i in 1..n1 - 1 {
        acc1[i] = (vl1[i + 1].0, acc1[i - 1].1 + vl1[i].1);
    }

    for i in 1..n2 - 1 {
        acc2[i] = (vl2[i + 1].0, acc2[i - 1].1 + vl2[i].1);
    }

    acc1[n1 - 1] = (usize::MAX, l);
    acc2[n2 - 1] = (usize::MAX, l);

    acc1.reverse();
    acc2.reverse();

    let mut m1 = vl1[0].0;
    let mut m2 = vl2[0].0;
    let mut cur = 0;
    let mut ans = 0;

    while !acc1.is_empty() || !acc2.is_empty() {
        match (acc1.last(), acc2.last()) {
            (Some((v1, p1)), Some((v2, p2))) => {
                if p1 <= p2 {
                    if m1 == m2 {
                        ans += p1 - cur;
                    }
                    m1 = *v1;
                    cur = *p1;
                    acc1.pop();
                } else {
                    if m1 == m2 {
                        ans += p2 - cur;
                    }
                    m2 = *v2;
                    cur = *p2;
                    acc2.pop();
                }
            }
            (Some((v1, p1)), None) => {
                if m1 == m2 {
                    ans += p1 - cur;
                }
                m1 = *v1;
                cur = *p1;
                acc1.pop();
            }
            (None, Some((v2, p2))) => {
                if m1 == m2 {
                    ans += p2 - cur;
                }
                m2 = *v2;
                cur = *p2;
                acc2.pop();
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}

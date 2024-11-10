#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering, Reverse};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut time = 0;
    let mut plants = VecDeque::new();

    for _ in 0..q {
        input! {
            k: usize,
        }

        if k == 1 {
            // 植えられた時の経過時間を記録する
            plants.push_back(time);
        } else if k == 2 {
            input! {
                t: usize,
            }
            // 時間経過
            time += t;
        } else {
            input! {
                h: usize,
            }
            let mut sum_p = 0;
            while !plants.is_empty() {
                // 現在の経過時間 - 植えられた時の経過時間が
                // この植物の高さ
                if time - plants.front().unwrap() >= h {
                    // 収穫する
                    sum_p += 1;
                    plants.pop_front();
                } else {
                    // 以降の植物もすべて高さが足りない
                    break;
                }
            }
            println!("{}", sum_p);
        }
    }
}

#[allow(dead_code)]
fn solve() {
    input! {
        q: usize,
    }

    let mut plants = vec![0];
    let mut times = vec![0];
    let mut harvest_pos = 0;

    for _ in 0..q {
        input! {
            k: usize,
        }

        if k == 1 {
            let i = plants.len() - 1;
            let j = times.len() - 1;
            if times[j] == 0 {
                plants[i] += 1;
            } else {
                plants.push(1);
                times.push(0);
            }
        } else if k == 2 {
            input! {
                t: usize,
            }
            let j = times.len() - 1;
            times[j] += t;
        } else {
            input! {
                h: usize,
            }
            if harvest_pos == plants.len() {
                println!("0");
                continue;
            }

            let mut sum_p = 0;
            let mut sum_t = 0;

            for i in (harvest_pos..plants.len()).rev() {
                if sum_t + times[i] < h {
                    sum_t += times[i];
                    harvest_pos = i;
                } else {
                    sum_p += plants[i];
                    sum_t += times[i];
                    harvest_pos = harvest_pos.max(i + 1);
                }
            }
            println!("{}", sum_p);
        }
    }
}

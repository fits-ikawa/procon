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
        h: usize, w: usize,
        image: [Chars; h],
    }

    let image = add_guard(image, '#');

    let mut check = hashset! {};
    let mut possibles = hashset! {};

    for i in 1..=h {
        for j in 1..=w {
            if image[i][j] == '#' {
                // 収縮結果としてあり得るパターンか調べる
                let mut all_black = true;
                let mut subcheck = hashset! {(i, j)};
                for k in 0..=8 {
                    let ii = i + k / 3 - 1;
                    let jj = j + k % 3 - 1;
                    all_black &= image[ii][jj] == '#';
                    subcheck.insert((ii, jj));
                }
                if all_black {
                    check.extend(subcheck);
                    possibles.insert((i, j));
                }
            }
        }
    }

    #[allow(clippy::needless_range_loop)]
    for i in 1..=h {
        for j in 1..=w {
            if image[i][j] == '#' && !check.contains(&(i, j)) {
                println!("impossible");
                return;
            }
        }
    }

    let ans = (1..=h)
        .map(|i| {
            (1..=w)
                .map(|j| {
                    if possibles.contains(&(i, j)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .join("\n");

    println!("possible");
    println!("{}", ans);
}

fn add_guard<T: Clone>(field: Vec<Vec<T>>, guard: T) -> Vec<Vec<T>> {
    assert!(!field.is_empty());
    assert!(!field[0].is_empty() && field.iter().map(|row| row.len()).all_equal());

    use std::iter::once;
    let width = field[0].len();

    once(vec![guard.clone(); width + 2])
        .chain(field.into_iter().map(|row| {
            once(guard.clone())
                .chain(row)
                .chain(once(guard.clone()))
                .collect()
        }))
        .chain(once(vec![guard.clone(); width + 2]))
        .collect()
}

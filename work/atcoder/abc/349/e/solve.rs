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

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    input! {
        a: [[isize; 3]; 3],
    }

    let color = [[Color::White; 3]; 3];
    let mut memo = hashmap! {};

    println!(
        "{}",
        if game(true, color, &a, &mut memo) {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Color {
    White,
    Red,
    Blue,
}

fn game(
    takahashi: bool,
    color: [[Color; 3]; 3],
    num: &[Vec<isize>],
    memo: &mut HashMap<(bool, [[Color; 3]; 3]), bool>,
) -> bool {
    if let Some(&ret) = memo.get(&(takahashi, color)) {
        return ret;
    }

    let mut check = || {
        let mut new_color = color;
        let my_color = if takahashi { Color::Red } else { Color::Blue };

        for i in 0..3 {
            for j in 0..3 {
                if color[i][j] == Color::White {
                    // マスを選んで色を塗る
                    new_color[i][j] = my_color;
                    if three(my_color, &new_color) {
                        return true;
                    }
                    if all_colored(&new_color) {
                        return point(takahashi, &new_color, num);
                    }
                    if !game(!takahashi, new_color, num, memo) {
                        return true;
                    }
                    // 次のチェックのために色を戻す
                    new_color[i][j] = Color::White;
                }
            }
        }
        // 勝てる手がない
        false
    };

    let ret = check();
    memo.insert((takahashi, color), ret);
    ret
}

fn three(c: Color, b: &[[Color; 3]; 3]) -> bool {
    (b[0][0] == c && b[0][1] == c && b[0][2] == c)
        || (b[1][0] == c && b[1][1] == c && b[1][2] == c)
        || (b[2][0] == c && b[2][1] == c && b[2][2] == c)
        || (b[0][0] == c && b[1][0] == c && b[2][0] == c)
        || (b[0][1] == c && b[1][1] == c && b[2][1] == c)
        || (b[0][2] == c && b[1][2] == c && b[2][2] == c)
        || (b[0][0] == c && b[1][1] == c && b[2][2] == c)
        || (b[0][2] == c && b[1][1] == c && b[2][0] == c)
}

fn point(takahashi: bool, color: &[[Color; 3]; 3], num: &[Vec<isize>]) -> bool {
    let mut t = 0;
    let mut a = 0;
    for i in 0..3 {
        for j in 0..3 {
            match color[i][j] {
                Color::Red => t += num[i][j],
                Color::Blue => a += num[i][j],
                _ => unreachable!(),
            }
        }
    }
    (takahashi && t > a) || (!takahashi && a > t)
}

#[allow(clippy::needless_range_loop)]
fn all_colored(color: &[[Color; 3]; 3]) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if color[i][j] == Color::White {
                return false;
            }
        }
    }
    true
}

#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use memoise::memoise_map;
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
        n: usize, m: usize, l: usize,
        mut a: [usize; n],
        mut b: [usize; m],
        mut c: [usize; l],
    }

    let mut owners = [Owner::Table; 12];
    let mut cards = vec![];

    for i in 0..n {
        owners[i] = Owner::Takahashi;
        cards.push(a[i]);
    }

    for i in 0..m {
        owners[n + i] = Owner::Aoki;
        cards.push(b[i]);
    }

    for i in 0..l {
        cards.push(c[i]);
    }

    let mut memo = hashmap! {};

    println!(
        "{}",
        if game(Turn::Takahashi, owners, &cards, &mut memo) {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Turn {
    Takahashi,
    Aoki,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Owner {
    Takahashi,
    Aoki,
    Table,
}

impl Turn {
    pub fn next(self) -> Self {
        match self {
            Turn::Takahashi => Turn::Aoki,
            Turn::Aoki => Turn::Takahashi,
        }
    }
}

impl From<Turn> for Owner {
    fn from(turn: Turn) -> Self {
        match turn {
            Turn::Takahashi => Owner::Takahashi,
            Turn::Aoki => Owner::Aoki,
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn game(
    turn: Turn,
    owners: [Owner; 12],
    cards: &[usize],
    memo: &mut HashMap<(Turn, [Owner; 12]), bool>,
) -> bool {
    if let Some(&ret) = memo.get(&(turn, owners)) {
        return ret;
    }

    let mut check = || {
        for i in 0..cards.len() {
            if owners[i] == turn.into() {
                // 手札から場に出す
                let mut new_owners = owners;
                new_owners[i] = Owner::Table;
                for j in 0..cards.len() {
                    if owners[j] == Owner::Table && cards[i] > cards[j] {
                        // 場から手札に取る
                        new_owners[j] = turn.into();
                        if !game(turn.next(), new_owners, cards, memo) {
                            return true;
                        }
                        // チェックが終わったので場に戻す
                        new_owners[j] = Owner::Table;
                    }
                }

                // 場札を取らないケース
                if !game(turn.next(), new_owners, cards, memo) {
                    return true;
                }
            }
        }
        // 勝てる手がなかった or 出せる手札がない
        false
    };

    let ret = check();
    memo.insert((turn, owners), ret);
    ret
}

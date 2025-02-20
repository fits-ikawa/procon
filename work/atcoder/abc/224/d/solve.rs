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
    // 盤面の配列をそのままキーに
    // 最速ケースはこっちの方が速く、最遅ケースはこっちの方が遅い
    input! {
        m: usize,
        uv: [(Usize1, Usize1); m],
        p: [Usize1; 8],
    }

    let mut adj = vec![vec![]; 9];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut board = vec![8_u8; 9];

    for i in 0..8 {
        board[p[i]] = i as u8;
    }

    let mut todo = VecDeque::new();
    let mut seen = hashmap! {};

    todo.push_back(board.clone());
    seen.insert(board, 0);

    while let Some(mut board) = todo.pop_front() {
        let cost = seen.get(&board).copied().unwrap();
        let pos = board.iter().position(|&i| i == 8).unwrap();

        for &next in &adj[pos] {
            board.swap(pos, next);
            if !seen.contains_key(&board) {
                seen.insert(board.clone(), cost + 1);
                todo.push_back(board.clone());
            }
            board.swap(pos, next);
        }
    }

    if let Some(ans) = seen.get(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[allow(dead_code)]
fn solve() {
    // 盤面を 9 進数エンコードしてキーに
    input! {
        m: usize,
        uv: [(Usize1, Usize1); m],
        p: [Usize1; 8],
    }

    let mut adj = vec![vec![]; 9];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    let enc = |pos: &Vec<usize>| -> usize {
        let mut code = 0;
        for i in 0..8 {
            code += pos[i] * 9_usize.pow(i as u32);
        }
        code
    };

    // let dec = |code: usize| -> Vec<usize> {
    //     let mut code = code;
    //     let mut ret = vec![0; 8];
    //     for i in 0..8 {
    //         ret[i] = code % 9;
    //         code /= 9;
    //     }
    //     ret
    // };

    // 有り得る盤面間の遷移を表す隣接リスト
    let mut adj2 = hashmap! {};

    for i in 0..9_usize {
        for mut perm in (0..9).filter(|&j| i != j).permutations(8) {
            let mut pos2num = [0; 9];
            for num in 0..8 {
                pos2num[perm[num]] = num;
            }

            let from = enc(&perm);
            let tovec = adj2.entry(from).or_insert(vec![]);

            for &j in &adj[i] {
                perm[pos2num[j]] = i;
                tovec.push(enc(&perm));
                perm[pos2num[j]] = j;
            }
        }
    }

    let start = enc(&p);

    let mut todo = VecDeque::new();
    let mut seen = hashmap! {};

    todo.push_back(start);
    seen.insert(start, 0);

    while let Some(from) = todo.pop_front() {
        let cost = seen.get(&from).copied().unwrap();
        for &to in adj2.get(&from).unwrap() {
            if !seen.contains_key(&to) {
                seen.insert(to, cost + 1);
                todo.push_back(to);
            }
        }
    }

    if let Some(ans) = seen.get(&enc(&vec![0, 1, 2, 3, 4, 5, 6, 7])) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

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
use std::rc::Rc;
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
        n: usize, k: usize,
        p: [Usize1; n],
    }

    let mut table = btreemap! {};
    let mut stack = vec![None; n];
    let mut ans = vec![-1; n];

    for (i, pi) in p.into_iter().enumerate() {
        let turn = (i + 1) as isize;
        if let Some((&top, &cnt)) = table.range(pi..).next() {
            if cnt + 1_usize >= k {
                // k 枚目を置く
                ans[pi] = turn;
                ans[top] = turn;
                let mut cur = stack[top];
                while let Some(next) = cur {
                    ans[next] = turn;
                    cur = stack[next];
                }
            } else {
                // k - 1 枚目までを置く
                stack[pi] = Some(top);
                table.insert(pi, cnt + 1);
            }
            table.remove(&top);
        } else if k == 1 {
            // 1 枚目を置いた瞬間食べられるケース
            ans[pi] = turn;
        } else {
            // 1 枚目を置く
            table.insert(pi, 1);
        }
    }

    println!("{}", ans.iter().join("\n"));
}

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
        n: usize,
        s: Chars,
        t: Chars,
    }

    let cnt_sb = s.iter().filter(|&&x| x == 'B').count();
    let cnt_tb = t.iter().filter(|&&x| x == 'B').count();

    if cnt_sb != cnt_tb {
        println!("-1");
        return;
    }

    let mut adj = hashmap! {};

    for i in 0..n + 1 {
        let mut b = (0..n + 2).collect::<HashSet<_>>();
        b.remove(&i);
        b.remove(&(i + 1));

        for comb in b.into_iter().combinations(cnt_sb) {
            let mut from = vec!['W'; n + 2];
            from[i] = '.';
            from[i + 1] = '.';
            for bi in comb {
                from[bi] = 'B';
            }

            let value = adj.entry(from.iter().collect::<String>()).or_insert(vec![]);

            for j in 0..n + 1 {
                if from[j] != '.' && from[j + 1] != '.' {
                    from.swap(i, j);
                    from.swap(i + 1, j + 1);
                    value.push(from.iter().collect::<String>());
                    from.swap(i, j);
                    from.swap(i + 1, j + 1);
                }
            }
        }
    }

    let mut start = s.clone();
    start.push('.');
    start.push('.');
    let start = start.iter().collect::<String>();

    let mut goal = t.clone();
    goal.push('.');
    goal.push('.');
    let goal = goal.iter().collect::<String>();

    let mut todo = VecDeque::new();
    let mut seen = hashmap! {};

    todo.push_back(start.clone());
    seen.insert(start, 0);

    while let Some(from) = todo.pop_front() {
        if from == goal {
            println!("{}", seen[&from]);
            return;
        }
        for to in &adj[&from] {
            if !seen.contains_key(to) {
                seen.insert(to.clone(), seen[&from] + 1);
                todo.push_back(to.clone());
            }
        }
    }

    println!("-1");
}

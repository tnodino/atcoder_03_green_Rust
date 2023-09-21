// https://atcoder.jp/contests/abc320/tasks/abc320_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;
use itertools::Itertools;

const INF: usize = 1<<60;

use std::cmp::Ordering;
fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (M, S1, S2, S3): (usize, String, String, String),
    }
    let S1 = S1.chars().collect::<Vec<char>>();
    let S2 = S2.chars().collect::<Vec<char>>();
    let S3 = S3.chars().collect::<Vec<char>>();
    let mut num = vec![vec![Vec::new(); 3]; 10];
    for i in 0..M {
        let idx = (S1[i] as usize) - ('0' as usize);
        for k in 0..5 {
            num[idx][0].push(i + M * k);
        }
        let idx = (S2[i] as usize) - ('0' as usize);
        for k in 0..5 {
            num[idx][1].push(i + M * k);
        }
        let idx = (S3[i] as usize) - ('0' as usize);
        for k in 0..5 {
            num[idx][2].push(i + M * k);
        }
    }
    for i in 0..10 {
        for j in 0..3 {
            num[i][j].sort();
        }
    }
    let mut ans: usize = INF;
    for i in 0..10 {
        if num[i][0].is_empty() || num[i][1].is_empty() || num[i][2].is_empty() {
            continue;
        }
        for perm in (0..3).permutations(3) {
            let mut res = 0;
            for j in 0..3 {
                let idx = bisect_left(&num[i][perm[j]], &res);
                res = num[i][perm[j]][idx] + 1;
            }
            ans = min(ans, res - 1);
        }
    }
    match ans {
        INF => println!("-1"),
        _ => println!("{}", ans),
    }
}
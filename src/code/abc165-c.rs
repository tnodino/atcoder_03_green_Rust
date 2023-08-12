// https://atcoder.jp/contests/abc165/tasks/abc165_c

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        Q: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..Q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        vec.push((a, b, c, d));
    }
    let mut que = VecDeque::new();
    que.push_back(vec![1]);
    let mut ans = 0;
    while !que.is_empty() {
        let mut x = que.pop_front().unwrap();
        if x.len() == N + 1 {
            let mut res = 0;
            for i in 0..Q {
                if x[vec[i].1] - x[vec[i].0] == vec[i].2 {
                    res += vec[i].3;
                }
            }
            ans = max(ans, res);
            continue;
        }
        let last = x[x.len()-1];
        for i in last..=M {
            x.push(i);
            que.push_back(x.clone());
            x.pop();
        }
    }
    println!("{}", ans);
}
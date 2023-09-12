// https://atcoder.jp/contests/abc114/tasks/abc114_c

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    let mut que = VecDeque::from([0]);
    while !que.is_empty() {
        let num = que.pop_front().unwrap() * 10;
        let x7 = num + 7;
        let x5 = num + 5;
        let x3 = num + 3;
        if x7 <= N {
            vec.push(x7);
            que.push_back(x7);
        }
        if x5 <= N {
            vec.push(x5);
            que.push_back(x5);
        }
        if x3 <= N {
            vec.push(x3);
            que.push_back(x3);
        }
    }
    let M = vec.len();
    let mut ans = 0;
    for i in 0..M {
        let mut num = vec[i];
        let mut flg = vec![false; 3];
        while num > 0 {
            let idx = (num % 10) / 2 - 1;
            flg[idx] = true;
            num /= 10;
        }
        if flg.iter().all(|&x| x) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
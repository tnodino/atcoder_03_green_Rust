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
    let arr = [3, 5, 7];
    let mut que = VecDeque::new();
    que.push_back(0);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for i in 0..3 {
            let x = pos * 10 + arr[i];
            if x > N {
                continue;
            }
            vec.push(x);
            que.push_back(x);
        }
    }
    let mut ans = 0;
    for i in 0..vec.len() {
        let mut x = vec[i];
        let mut flg = [false; 3];
        while x > 0 {
            let idx = match x % 10 {
                3 => 0,
                5 => 1,
                _ => 2,
            };
            x /= 10;
            flg[idx] = true;
        }
        if flg.iter().all(|&x| x) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
// https://atcoder.jp/contests/abc064/tasks/abc064_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut que = VecDeque::new();
    let mut cnt = 0;
    for i in 0..N {
        match S[i] {
            '(' => cnt += 1,
            _ => cnt -= 1,
        }
        if cnt == -1 {
            que.push_front('(');
            cnt = 0;
        }
        que.push_back(S[i]);
    }
    for _ in 0..cnt {
        que.push_back(')');
    }
    println!("{}", que.iter().collect::<String>());
}
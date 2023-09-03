// https://atcoder.jp/contests/abc235/tasks/abc235_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        N: usize,
    }
    let M = 10;
    let mut pow10 = vec![1; M];
    for i in 1..M {
        pow10[i] = pow10[i-1] * 10;
    }
    let mut K = 0;
    for i in 0..M {
        if N < pow10[i] {
            K = pow10[i];
            break;
        }
    }
    let mut que = VecDeque::new();
    que.push_back(1);
    let mut cost = vec![-1; K+1];
    cost[1] = 0;
    while !que.is_empty() {
        let x = que.pop_front().unwrap();
        let c = x * a;
        if c <= K && cost[c] == -1 {
            cost[c] = cost[x] + 1;
            que.push_back(c);
        }
        if x % 10 == 0 {
            continue;
        }
        let mut c = x / 10;
        for i in 0..M {
            if c / pow10[i] == 0 {
                c += (x % 10) * pow10[i];
                break;
            }
        }
        if c <= K && cost[c] == -1 {
            cost[c] = cost[x] + 1;
            que.push_back(c);
        }
    }
    println!("{}", cost[N]);
}
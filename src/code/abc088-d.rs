// https://atcoder.jp/contests/abc088/tasks/abc088_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[0, 0, !0, 1];
const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut s = Vec::new();
    let mut w = 0;
    for _ in 0..H {
        input! {
            t: String,
        }
        let t = t.chars().collect::<Vec<char>>();
        w += t.iter().filter(|&x| *x == '.').count();
        s.push(t);
    }
    let mut cost = vec![vec![INF; W]; H];
    cost[0][0] = 1;
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    que.push_back((0, 0));
    while !que.is_empty() {
        let (x, y) = que.pop_front().unwrap();
        for d in 0..4 {
            let nx = x.wrapping_add(DX[d]);
            let ny = y.wrapping_add(DY[d]);
            if H <= nx || W <= ny {
                continue;
            }
            if s[nx][ny] == '#' {
                continue;
            }
            if cost[x][y] + 1 < cost[nx][ny] {
                cost[nx][ny] = cost[x][y] + 1;
                que.push_back((nx, ny));
            }
        }
    }
    if cost[H-1][W-1] == INF {
        println!("-1");
    }
    else {
        println!("{}", w - cost[H-1][W-1]);
    }
}
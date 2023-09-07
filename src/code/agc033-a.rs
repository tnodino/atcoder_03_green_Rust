// https://atcoder.jp/contests/agc033/tasks/agc033_a

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[ 0, 0,!0, 1];
const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
    }
    let mut A = Vec::new();
    for _ in 0..H {
        input! {
            a: String,
        }
        let a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut cost = vec![vec![INF; W]; H];
    let mut que = VecDeque::new();
    for i in 0..H {
        for j in 0..W {
            if A[i][j] == '#' {
                cost[i][j] = 0;
                que.push_back((i, j));
            }
        }
    }
    while !que.is_empty() {
        let (x, y) = que.pop_front().unwrap();
        for d in 0..4 {
            let nx = x.wrapping_add(DX[d]);
            let ny = y.wrapping_add(DY[d]);
            if H <= nx || W <= ny {
                continue;
            }
            if cost[x][y] + 1 < cost[nx][ny] {
                cost[nx][ny] = cost[x][y] + 1;
                que.push_back((nx, ny));
            }
        }
    }
    println!("{}", cost.iter().flat_map(|x| x.iter()).max().unwrap());
}
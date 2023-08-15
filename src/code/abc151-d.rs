// https://atcoder.jp/contests/abc151/tasks/abc151_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;
use std::cmp::max;

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
    let mut S = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '.' {
                let mut cost = vec![vec![INF; W]; H];
                cost[i][j] = 0;
                let mut que = VecDeque::new();
                que.push_back((i, j));
                while !que.is_empty() {
                    let (x, y) = que.pop_front().unwrap();
                    for d in 0..4 {
                        let nx = x.wrapping_add(DX[d]);
                        let ny = y.wrapping_add(DY[d]);
                        if H <= nx || W <= ny {
                            continue;
                        }
                        if S[nx][ny] == '#' {
                            continue;
                        }
                        if cost[x][y] + 1 < cost[nx][ny] {
                            cost[nx][ny] = cost[x][y] + 1;
                            que.push_back((nx, ny));
                            ans = max(ans, cost[nx][ny]);
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
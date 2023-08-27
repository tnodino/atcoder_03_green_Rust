// https://atcoder.jp/contests/abc317/tasks/abc317_e

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
        H: usize,
        W: usize,
    }
    let mut A = Vec::new();
    for _ in 0..H {
        input! {
            a: String,
        }
        let a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut sx = 0;
    let mut sy = 0;
    let mut gx = 0;
    let mut gy = 0;
    for i in 0..H {
        for j in 0..W {
            if A[i][j] == 'S' {
                sx = i;
                sy = j;
                A[i][j] = '.';
            }
            else if A[i][j] == 'G' {
                gx = i;
                gy = j;
                A[i][j] = '.';
            }
            else if A[i][j] == '.' || A[i][j] == '#' || A[i][j] == 'x' {
                continue;
            }
            else {
                let idx = match A[i][j] {
                    '^' => 0,
                    'v' => 1,
                    '<' => 2,
                    _ => 3,
                };
                let mut x = i;
                let mut y = j;
                loop {
                    let nx = x.wrapping_add(DX[idx]);
                    let ny = y.wrapping_add(DY[idx]);
                    if H <= nx || W <= ny {
                        break;
                    }
                    if A[nx][ny] != '.' && A[nx][ny] != 'x' {
                        break;
                    }
                    A[nx][ny] = 'x';
                    x = nx;
                    y = ny;
                }
                A[i][j] = '#';
            }
        }
    }
    for i in 0..H {
        for j in 0..W {
            if A[i][j] == 'x' {
                A[i][j] = '#';
            }
        }
    }
    let mut cost = vec![vec![INF; W]; H];
    cost[sx][sy] = 0;
    let mut que = VecDeque::new();
    que.push_back((sx, sy));
    while !que.is_empty() {
        let (x, y) = que.pop_front().unwrap();
        for d in 0..4 {
            let nx = x.wrapping_add(DX[d]);
            let ny = y.wrapping_add(DY[d]);
            if H <= nx || W <= ny {
                continue;
            }
            if A[nx][ny] == '#' {
                continue;
            }
            if cost[x][y] + 1 < cost[nx][ny] {
                cost[nx][ny] = cost[x][y] + 1;
                que.push_back((nx, ny));
            }
        }
    }
    if cost[gx][gy] == INF {
        println!("-1");
    }
    else {
        println!("{}", cost[gx][gy]);
    }
}
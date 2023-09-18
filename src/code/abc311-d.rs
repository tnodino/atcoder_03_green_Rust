// https://atcoder.jp/contests/abc311/tasks/abc311_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[ 0, 0,!0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut G = vec![vec![vec![Vec::new(); 5]; M]; N];
    for i in 0..N {
        for j in 0..M {
            if S[i][j] == '#' {
                continue;
            }
            for d in 0..4 {
                let nx = i.wrapping_add(DX[d]);
                let ny = j.wrapping_add(DY[d]);
                if N <= nx || M <= ny {
                    continue;
                }
                if S[nx][ny] == '#' {
                    G[i][j][d].push((i, j, 4));
                }
                else {
                    G[i][j][4].push((i, j, d));
                    G[i][j][d].push((nx, ny, d));
                }
            }
        }
    }
    let mut flg = vec![vec![vec![false; 5]; M]; N];
    flg[1][1][4] = true;
    let mut que = VecDeque::from([(1, 1, 4)]);
    while !que.is_empty() {
        let (x, y, from) = que.pop_front().unwrap();
        for (nx, ny, to) in G[x][y][from].iter() {
            if !flg[*nx][*ny][*to] {
                flg[*nx][*ny][*to] = true;
                que.push_back((*nx, *ny, *to));
            }
        }
    }
    let mut ans = 0;
    for i in 0..N {
        for j in 0..M {
            ans += flg[i][j].iter().any(|&x| x) as usize;
        }
    }
    println!("{}", ans);
}
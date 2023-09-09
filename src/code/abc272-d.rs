// https://atcoder.jp/contests/abc272/tasks/abc272_d

use proconio::input;
use proconio::fastout;
use num::pow;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut root = vec![-1; M+1];
    for i in 0..=M {
        if i * i > M {
            break;
        }
        root[i*i] = i as isize;
    }
    let n = N as isize;
    let m = M as isize;
    let mut G = vec![vec![Vec::new(); N]; N];
    for i in 0..n {
        for k in 0..n {
            if pow(k-i, 2) > m {
                continue;
            }
            let r = root[(m - pow(k-i, 2)) as usize];
            if r == -1 {
                continue;
            }
            for j in 0..n {
                let l = j + r;
                if l < n {
                    G[i as usize][j as usize].push((k as usize, l as usize));
                }
                let l = j - r;
                if 0 <= l {
                    G[i as usize][j as usize].push((k as usize, l as usize));
                }
            }
        }
    }
    let mut cnt = vec![vec![-1; N]; N];
    cnt[0][0] = 0;
    let mut que = VecDeque::from([(0, 0)]);
    while !que.is_empty() {
        let (x, y) = que.pop_front().unwrap();
        for (nx, ny) in G[x][y].iter() {
            if cnt[*nx][*ny] == -1 {
                cnt[*nx][*ny] = cnt[x][y] + 1;
                que.push_back((*nx, *ny));
            }
        }
    }
    for i in 0..N {
        println!("{}", cnt[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}
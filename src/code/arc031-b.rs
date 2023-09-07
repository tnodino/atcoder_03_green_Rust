// https://atcoder.jp/contests/arc031/tasks/arc031_2

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[ 0, 0,!0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 10;
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            a: String,
        }
        let a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut a = vec![vec![false; N]; N];
    for i in 0..N {
        for j in 0..N {
            if A[i][j] == 'x' {
                a[i][j] = true;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            let mut flg = a.clone();
            flg[i][j] = false;
            let mut que = VecDeque::from([(i, j)]);
            while !que.is_empty() {
                let (x, y) = que.pop_front().unwrap();
                for d in 0..4 {
                    let nx = x.wrapping_add(DX[d]);
                    let ny = y.wrapping_add(DY[d]);
                    if N <= nx || N <= ny {
                        continue;
                    }
                    if !flg[nx][ny] {
                        flg[nx][ny] = true;
                        que.push_back((nx, ny));
                    }
                }
            }
            if flg.iter().flat_map(|x| x.iter()).all(|&x| x) {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}
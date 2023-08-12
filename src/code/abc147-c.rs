// https://atcoder.jp/contests/abc147/tasks/abc147_c

use proconio::input;
use std::cmp::max;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = vec![0; N];
    let mut vec = vec![Vec::new(); N];
    for i in 0..N {
        input! {
            a: usize,
        }
        A[i] = a;
        for _ in 0..a {
            input! {
                x: usize,
                y: usize,
            }
            vec[i].push((x-1, y));
        }
    }
    let mut ans = 0;
    'outer: for bit in 1..1<<N {
        let mut cnt = 0;
        let mut flg = vec![0; N];
        for i in 0..N {
            if bit & (1 << i) > 0 {
                cnt += 1;
                flg[i] = 1;
            }
        }
        for i in 0..N {
            if flg[i] == 1 {
                for j in 0..A[i] {
                    if flg[vec[i][j].0] != vec[i][j].1 {
                        continue 'outer;
                    }
                }
            }
        }
        ans = max(ans, cnt);
    }
    println!("{}", ans);
}
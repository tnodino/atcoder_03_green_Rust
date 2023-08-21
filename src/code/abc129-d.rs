// https://atcoder.jp/contests/abc129/tasks/abc129_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

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
    let mut cntu = vec![vec![1; W]; H];
    let mut cntl = vec![vec![1; W]; H];
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '#' {
                cntu[i][j] = 0;
                cntl[i][j] = 0;
                continue;
            }
            if i + 1 < H {
                cntu[i+1][j] = cntu[i][j] + 1;
            }
            if j + 1 < W {
                cntl[i][j+1] = cntl[i][j] + 1;
            }
        }
    }
    let mut cntd = vec![vec![1; W]; H];
    let mut cntr = vec![vec![1; W]; H];
    for i in (0..H).rev() {
        for j in (0..W).rev() {
            if S[i][j] == '#' {
                cntd[i][j] = 0;
                cntr[i][j] = 0;
                continue;
            }
            if 0 < i {
                cntd[i-1][j] = cntd[i][j] + 1;
            }
            if 0 < j {
                cntr[i][j-1] = cntr[i][j] + 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '#' {
                continue;
            }
            ans = max(ans, cntu[i][j] + cntl[i][j] + cntd[i][j] + cntr[i][j] - 3);
        }
    }
    println!("{}", ans);
}
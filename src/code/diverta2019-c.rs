// https://atcoder.jp/contests/diverta2019/tasks/diverta2019_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ba = 0;
    let mut xa = 0;
    let mut bx = 0;
    let mut ans = 0;
    for _ in 0..N {
        input! {
            S: String,
        }
        let M = S.len();
        let S = S.chars().collect::<Vec<char>>();
        for i in 0..M-1 {
            if S[i] == 'A' && S[i+1] == 'B' {
                ans += 1;
            }
        }
        match (S[0], S[M-1]) {
            ('B', 'A') => ba += 1,
            (_, 'A') => xa += 1,
            ('B', _) => bx += 1,
            (_, _) => {},
        }
    }
    if ba >= 1 {
        ans += ba - 1;
        if xa >= 1 {
            ans += 1;
            xa -= 1;
        }
        if bx >= 1 {
            ans += 1;
            bx -= 1;
        }
    }
    ans += min(xa, bx);
    println!("{}", ans);
}
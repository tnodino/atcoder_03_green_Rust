// https://atcoder.jp/contests/tenka1-2019/tasks/tenka1_2019_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut black = 0;
    for i in 0..N {
        if S[i] == '#' {
            black += 1;
        }
    }
    let mut white = 0;
    let mut ans = black;
    for i in (0..N).rev() {
        match S[i] {
            '#' => black -= 1,
            '.' => white += 1,
            _ => {},
        }
        ans = min(ans, black + white);
    }
    println!("{}", ans);
}
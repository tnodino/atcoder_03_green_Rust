// https://atcoder.jp/contests/abc049/tasks/arc065_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().rev().collect::<Vec<char>>();
    let A = ["dream", "dreamer", "erase", "eraser"];
    let mut T = Vec::new();
    for a in A.iter() {
        T.push(a.chars().rev().collect::<Vec<char>>());
    }
    let mut idx = 0;
    while idx < N {
        for i in 0..5 {
            if i == 4 {
                println!("NO");
                return;
            }
            if idx + T[i].len() > N {
                continue;
            }
            let mut flg = true;
            for j in 0..T[i].len() {
                if S[idx+j] != T[i][j] {
                    flg = false;
                }
            }
            if flg {
                idx += T[i].len();
                break;
            }
        }
    }
    println!("YES");
}
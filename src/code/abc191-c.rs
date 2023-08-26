// https://atcoder.jp/contests/abc191/tasks/abc191_c

use proconio::input;
use proconio::fastout;

fn f(x: char) -> usize {
    return match x {
        '#' => 1,
        _ => 0,
    };
}

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
    for i in 0..H-1 {
        for j in 0..W-1 {
            let mut cnt = 0;
            cnt += f(S[i][j]);
            cnt += f(S[i+1][j]);
            cnt += f(S[i][j+1]);
            cnt += f(S[i+1][j+1]);
            if cnt % 2 == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
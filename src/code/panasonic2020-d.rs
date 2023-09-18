// https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_d

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(s: String, ma: u8, N: usize) {
    if s.len() == N {
        println!("{}", s);
        return;
    }
    let mi = 'a' as u8;
    for c in mi..=ma {
        let t = format!("{}{}", s, c as char);
        match c == ma {
            true => dfs(t, ma + 1, N),
            false => dfs(t, ma, N),
        }
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    dfs("".to_string(), 'a' as u8, N);
}
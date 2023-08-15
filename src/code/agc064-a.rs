// https://atcoder.jp/contests/agc064/tasks/agc064_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let mut ans = Vec::new();
    for i in 1..=N-2 {
        ans.push(i);
    }
    ans.push(N);
    ans.push(N-1);
    while N >= 3 {
        for _ in 0..N-2 {
            ans.push(N);
            ans.push(N-1);
        }
        ans.push(N);
        N -= 2;
    }
    if N == 2 {
        ans.push(2);
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}
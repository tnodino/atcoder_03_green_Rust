// https://atcoder.jp/contests/agc011/tasks/agc011_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        C: usize,
        K: usize,
        mut T: [usize; N],
    }
    T.sort();
    let mut cnt = 1;
    let mut time = T[0];
    let mut ans = 1;
    for i in 1..N {
        if time + K < T[i] || cnt == C {
            cnt = 0;
            time = T[i];
            ans += 1;
        }
        cnt += 1;
    }
    println!("{}", ans);
}
// https://atcoder.jp/contests/abc027/tasks/abc027_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N],
    }
    let su = a.iter().sum::<usize>();
    if su % N > 0 {
        println!("-1");
        return;
    }
    let M = su / N;
    let mut ans = 0;
    let mut cnt = 1;
    for i in 0..N-1 {
        if M * cnt == a[i] {
            cnt = 1;
        }
        else {
            cnt += 1;
            ans += 1;
            a[i+1] += a[i];
        }
    }
    println!("{}", ans);
}
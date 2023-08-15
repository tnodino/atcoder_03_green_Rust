// https://atcoder.jp/contests/abc133/tasks/abc133_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut s = A.iter().sum::<usize>();
    for i in (1..N).step_by(2) {
        s -= A[i] * 2;
    }
    let mut ans = vec![0; N];
    ans[0] = s;
    for i in 1..N {
        ans[i] = A[i-1] * 2 - ans[i-1];
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}
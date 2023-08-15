// https://atcoder.jp/contests/abc102/tasks/arc100_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let mut vec = Vec::new();
    for i in 0..N {
        vec.push(A[i] - (i as isize + 1));
    }
    vec.sort();
    let mid;
    if N % 2 == 0 {
        mid = (vec[N/2-1] + vec[N/2]) / 2
    }
    else {
        mid = vec[N/2];
    }
    let mut ans = 0;
    for i in 0..N {
        ans += (A[i] - (mid + i as isize + 1)).abs();
    }
    println!("{}", ans);
}
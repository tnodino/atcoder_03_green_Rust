// https://atcoder.jp/contests/jsc2019-qual/tasks/jsc2019_qual_b

use proconio::input;
use proconio::fastout;
use ac_library::ModInt1000000007 as Mint;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, Mint),
        A: [usize; N],
    }
    let mut ans = Mint::new(0);
    for i in 0..N {
        let mut inv = Mint::new(0);
        let mut cnt = Mint::new(0);
        for j in 0..i {
            if A[i] > A[j] {
                cnt += 1;
            }
        }
        for j in i..N {
            if A[i] > A[j] {
                inv += 1;
                cnt += 1;
            }
        }
        ans += (inv * K) + ((K - 1) * K / 2 * cnt);
    }
    println!("{}", ans);
}
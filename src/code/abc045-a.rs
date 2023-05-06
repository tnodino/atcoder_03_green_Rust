// https://atcoder.jp/contests/abc045/tasks/arc061_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let N = s.len() - 1;
    let mut S = Vec::new();
    for c in s.chars() {
        S.push(c as usize - 48);
    }
    let mut ans = 0;
    for bit in 0..1<<N {
        let mut res = 0;
        let mut x = S[0];
        for i in 0..N {
            if bit & (1 << i) > 0 {
                res += x;
                x = 0;
            }
            x *= 10;
            x += S[i+1];
        }
        res += x;
        ans += res;
    }
    println!("{}", ans);
}
// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = 0;
    for n in 0..1000 {
        let mut num = Vec::new();
        num.push((('0' as u8) + ((n / 100) as u8)) as char);
        num.push((('0' as u8) + ((n / 10 % 10) as u8)) as char);
        num.push((('0' as u8) + ((n % 10) as u8)) as char);
        let mut idx = 0;
        for i in 0..N {
            if S[i] == num[idx] {
                idx += 1;
            }
            if idx == 3 {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
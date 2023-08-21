// https://atcoder.jp/contests/abc110/tasks/abc110_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut flgs = vec!['?'; 26];
    let mut flgt = vec!['?'; 26];
    for i in 0..N {
        let idx = (S[i] as usize) - ('a' as usize);
        if flgs[idx] == '?' {
            flgs[idx] = T[i];
        }
        else if flgs[idx] != T[i] {
            println!("No");
            return;
        }
        let idx = (T[i] as usize) - ('a' as usize);
        if flgt[idx] == '?' {
            flgt[idx] = S[i];
        }
        else if flgt[idx] != S[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
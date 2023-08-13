// https://atcoder.jp/contests/abc076/tasks/abc076_c

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let N = S.len();
    let M = T.len();
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    if N < M {
        println!("UNRESTORABLE");
        return;
    }
    let mut ans = Vec::new();
    'outer: for i in 0..=N-M {
        let mut s = S.clone();
        for j in 0..M {
            if s[i+j] != '?' && s[i+j] != T[j] {
                continue 'outer;
            }
            s[i+j] = T[j];
        }
        for j in 0..N {
            if s[j] == '?' {
                s[j] = 'a';
            }
        }
        let s = s.iter().collect::<String>();
        ans.push(s);
    }
    ans.sort();
    if ans.len() == 0 {
        println!("UNRESTORABLE");
    }
    else {
        println!("{}", ans[0]);
    }
}